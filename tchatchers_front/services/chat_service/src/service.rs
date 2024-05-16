// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use core::panic;

use futures::{SinkExt, StreamExt};
use gloo_console::error;
use gloo_net::websocket::{futures::WebSocket, Message, WebSocketError};
use serde::{Deserialize, Serialize};
use tchatchers_core::ws_message::WsMessage;
use tokio::pin;
use yew_agent::reactor::{reactor, ReactorScope};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum WebSocketReactorControl {
    Open(String),
    Send(WsMessage),
    Reconnect,
    Close,
}

#[reactor(ChatReactor)]
pub async fn websocket_reactor(mut scope: ReactorScope<WebSocketReactorControl, WsMessage>) {
    gloo_console::debug!("ChatReactor started");
    let _ = scope.flush().await;
    let (mut sender, mut reader) = scope.split();

    if let Some(m) = reader.next().await {
        let WebSocketReactorControl::Open(address) = m else {
            panic!("Opening message not received");
        };
        let Ok(websocket) = WebSocket::open(&address) else {
            gloo_console::error!("An error has been met while trying to open WS connection");
            let _ = sender.send(WsMessage::Close);
            return;
        };

        let (mut write, mut read) = websocket.split();
        let (in_tx, mut in_rx) = tokio::sync::broadcast::channel::<WsMessage>(2);

        let write_tsk = async move {
            while let Ok(s) = in_rx.recv().await {
                write
                    .send(Message::Bytes(serde_json::to_vec(&s).unwrap()))
                    .await
                    .unwrap();
            }
        };

        let read_tsk = async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(data)) => {
                        let Ok(msg): Result<WsMessage, _> = serde_json::from_slice(data.as_bytes())
                        else {
                            let _ = sender.send(WsMessage::SerializationError).await;
                            break;
                        };
                        let _ = sender.send(msg).await;
                    }
                    Ok(Message::Bytes(b)) => {
                        let Ok(msg): Result<WsMessage, _> = serde_json::from_slice(&b) else {
                            let _ = sender.send(WsMessage::SerializationError).await;
                            break;
                        };
                        let _ = sender.send(msg).await;
                    }
                    Err(e) => match e {
                        WebSocketError::ConnectionError => {
                            error!("Error on connection");
                            let _ = sender.send(WsMessage::ClientDisconnected).await;
                        }
                        WebSocketError::ConnectionClose(e) => {
                            error!("The connection has been closed :", e.code);
                            error!("Error :", e.reason);
                            let _ = sender.send(WsMessage::ConnectionClosed).await;
                        }
                        WebSocketError::MessageSendError(e) => {
                            error!("Error while sending message", e.to_string());
                            let _ = sender.send(WsMessage::ErrorOnMessage(e.to_string())).await;
                        }
                        _ => error!("Unexpected error while communicating with distant ws"),
                    },
                }
            }
        };

        let scope_bridge_tsk = async {
            while let Some(m) = reader.next().await {
                match m {
                    WebSocketReactorControl::Send(m) => {
                        let _ = in_tx.send(m);
                    }
                    WebSocketReactorControl::Reconnect => {}
                    _ => break,
                }
            }
        };

        pin!(read_tsk, write_tsk, scope_bridge_tsk);

        tokio::select! {
            _ = (&mut read_tsk) => {
                gloo_console::debug!("Read aborted");
            },
            _ = (&mut write_tsk) => {
                gloo_console::debug!("Write aborted");
            },
            _ = (&mut scope_bridge_tsk) => gloo_console::debug!("End of scope bridge")
        }
    } else {
        gloo_console::debug!("No message received, skipped");
    }
    gloo_console::debug!("Exiting");
}
