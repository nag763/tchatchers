// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! The websocket is used to communicate between users within rooms.
//!
//! Websockets are isolated to each others, with one existing for each room.

use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::AppState;
use axum::{
    extract::{ws::Message, ws::WebSocket, Path, State, WebSocketUpgrade},
    response::IntoResponse,
};
use futures_util::{join, SinkExt, StreamExt};
use tchatchers_core::{
    api_response::ApiGenericResponse,
    async_message::AsyncMessage,
    room::RoomNameValidator,
    ws_message::{WsMessage, WsMessageContent, WsReceptionStatus},
};
use tokio::sync::broadcast;
use validator::Validate;

/// Hashmap that contains the room name as key and the websocket data as value.
#[derive(Default, Debug)]
pub struct WsRooms(HashMap<String, broadcast::Sender<Vec<u8>>>);

impl Deref for WsRooms {
    type Target = HashMap<String, broadcast::Sender<Vec<u8>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WsRooms {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// The HTTP entry point.
///
/// # Arguments
///
/// - ws : The 'Upgrade' header, mandatory.
/// - state : The data shared across threads, used to retrieve existing rooms.
/// - room : the room name.
/// - jwt : The authenticated user infos.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(room): Path<String>,
) -> Result<impl IntoResponse, ApiGenericResponse> {
    let room_name_validator: RoomNameValidator = RoomNameValidator::from(room.clone());
    if let Err(e) = room_name_validator.validate() {
        return Err(ApiGenericResponse::from(e));
    }
    Ok(ws.on_upgrade(|socket| handle_socket(socket, state, room)))
}

/// The socket handler
///
/// # Arguments
///
/// - socket : The struct used to communicate between the client and the server.
/// - state : The data shared across threads.
/// - room : The room name.
/// - user : The connected user's infos.
async fn handle_socket(socket: WebSocket, state: AppState, room: String) {
    let (mut sender, mut receiver) = socket.split();
    let tx = {
        let mut rooms = state.txs.lock().await;
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
            if sender.send(Message::Binary(msg)).await.is_err() {
                break;
            }
        }
    });

    // This task will receive messages from client and send them to broadcast subscribers.
    let mut recv_task: tokio::task::JoinHandle<Result<(), ApiGenericResponse>> =
        tokio::spawn(async move {
            while let Some(Ok(Message::Binary(text))) = receiver.next().await {
                if let Ok(msg) = serde_json::from_slice(&text) {
                    match msg {
                        WsMessage::Close => break,
                        WsMessage::Ping => {
                            let _ = tx.send(serde_json::to_vec(&WsMessage::Pong)?);
                        }
                        WsMessage::Pong | WsMessage::ClientKeepAlive => continue,
                        WsMessage::Send(mut ws_message) => {
                            ws_message.reception_status = WsReceptionStatus::Sent;
                            let redis_conn = state.async_pool.clone();
                            let _ = tx
                                .send(serde_json::to_vec(&WsMessage::Receive(ws_message.clone()))?);
                            tokio::spawn(async move {
                                let (mut pool1, mut pool2) =
                                    (redis_conn.clone(), redis_conn.clone());
                                join!(
                                    AsyncMessage::CleanRoom(ws_message.clone().room)
                                        .spawn(&mut pool1),
                                    AsyncMessage::PersistMessage(ws_message).spawn(&mut pool2)
                                );
                                anyhow::Ok(())
                            });
                        }
                        WsMessage::RetrieveMessages(session_id) => {
                            let messages: Vec<WsMessageContent> =
                                WsMessageContent::query_all_for_room(&room, &state.pg_pool).await?;
                            let _ = tx.send(serde_json::to_vec(&WsMessage::MessagesRetrieved {
                                messages,
                                session_id,
                            })?);
                        }
                        WsMessage::Seen(messages) => {
                            let _ = tx.send(serde_json::to_vec(&WsMessage::MessagesSeen(
                                messages.clone(),
                            ))?);
                            let redis_pool = state.async_pool.clone();
                            for message in messages.into_iter() {
                                let mut redis_pool = redis_pool.clone();
                                std::mem::drop(tokio::task::spawn(async move {
                                    AsyncMessage::spawn(
                                        AsyncMessage::MessageSeen(message),
                                        &mut redis_pool,
                                    )
                                    .await;
                                    anyhow::Ok(())
                                }));
                            }
                        }
                        _ => {}
                    }
                }
            }
            Ok(())
        });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
