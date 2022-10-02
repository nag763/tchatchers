use crate::State;
use crate::JWT_PATH;
use axum::{
    extract::{ws::Message, ws::WebSocket, Path, WebSocketUpgrade},
    response::{IntoResponse, Redirect},
    Extension,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tchatchers_core::{
    jwt::Jwt,
    room::Room,
    user::PartialUser,
    ws_message::{WsMessage, WsMessageType},
};
use tokio::sync::broadcast;
use tower_cookies::Cookies;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<State>>,
    Path(room): Path<String>,
    cookies: Cookies,
) -> impl IntoResponse {
    if let Some(cookie) = cookies.get(JWT_PATH) {
        if let Ok(jwt) = Jwt::deserialize(&cookie.value(), &state.jwt_secret) {
            ws.on_upgrade(|socket| handle_socket(socket, state, room, jwt.user))
        } else {
            Redirect::to("/logout").into_response()
        }
    } else {
        Redirect::to("/signin").into_response()
    }
}

async fn handle_socket(socket: WebSocket, state: Arc<State>, room: String, user: PartialUser) {
    let (mut sender, mut receiver) = socket.split();
    let tx = {
        let mut rooms = state.txs.lock().unwrap();
        match rooms.get(&room) {
            Some(v) => v.clone(),
            None => {
                let (tx, _rx) = broadcast::channel(1000);
                rooms.insert(room.clone(), tx.clone());
                tx
            }
        }
    };
    let mut rx = tx.subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // This task will receive messages from client and send them to broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Add username before message.
            match text.as_str() {
                "Close" => {
                    break;
                }
                "Ping" => {
                    let _ = tx.send(String::from("Pong"));
                }
                "Pong" => {
                    continue;
                }
                t => {
                    let msg: WsMessage = serde_json::from_str(t).unwrap();
                    match msg.message_type {
                        WsMessageType::Send => {
                            let ws_message = WsMessage {
                                message_type: WsMessageType::Receive,
                                content: msg.content,
                                author: msg.author,
                                room: Some(room.clone()),
                                ..WsMessage::default()
                            };
                            let _ = tx.send(serde_json::to_string(&ws_message).unwrap());

                            Room::publish_message_in_room(
                                &mut state.redis_pool.get().unwrap(),
                                &room,
                                ws_message.clone(),
                            );
                        }
                        WsMessageType::RetrieveMessages => {
                            let msgs = Room::find_messages_in_room(
                                &mut state.redis_pool.get().unwrap(),
                                &room,
                            );
                            let author = msg.author;
                            for mut retrieved_msg in msgs {
                                retrieved_msg.to = author.clone();
                                let _ = tx.send(serde_json::to_string(&retrieved_msg).unwrap());
                            }
                            let ws_message = WsMessage {
                                message_type: WsMessageType::MessagesRetrieved,
                                author: Some(user.clone().into()),
                                ..WsMessage::default()
                            };
                            let _ = tx.send(serde_json::to_string(&ws_message).unwrap());
                        }
                        _ => {}
                    }
                }
            };
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
