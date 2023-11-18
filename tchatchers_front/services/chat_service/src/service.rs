// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use gloo_console::{debug, error};
use gloo_net::websocket::{futures::WebSocket, Message, WebSocketError};
use tchatchers_core::ws_message::WsMessage;
use wasm_bindgen_futures::spawn_local;
use yew_agent::Dispatched;

use crate::bus::ChatBus;

#[derive(Clone, Debug)]
pub struct WebsocketService {
    pub tx: Sender<WsMessage>,
}

impl WebsocketService {
    pub fn new(room: &str) -> Self {
        let location = web_sys::window().unwrap().location();
        let host = location.host().unwrap();
        let protocol = location.protocol().unwrap();
        let ws_protocol = match protocol.as_str() {
            "https:" => "wss:",
            _ => "ws:",
        };
        let ws_addr = format!(
            "{}//{}/ws/{}?_={}",
            ws_protocol,
            host,
            room,
            js_sys::Date::new_0().get_time()
        );
        let ws = WebSocket::open(&ws_addr).unwrap();

        let (mut write, mut read) = ws.split();

        let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<WsMessage>(1000);
        let mut event_bus = ChatBus::dispatcher();

        spawn_local(async move {
            while let Some(s) = in_rx.next().await {
                write
                    .send(Message::Bytes(serde_json::to_vec(&s).unwrap()))
                    .await
                    .unwrap();
            }
        });

        spawn_local(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(data)) => {
                        if let Ok(msg) = serde_json::from_slice(data.as_bytes()) {
                            event_bus.send(msg);
                        }
                    }
                    Ok(Message::Bytes(b)) => {
                        if let Ok(msg) = serde_json::from_slice(&b) {
                            event_bus.send(msg);
                        }
                    }
                    Err(e) => match e {
                        WebSocketError::ConnectionError => {
                            error!("Error on connection");
                            event_bus.send(WsMessage::ClientDisconnected);
                        }
                        WebSocketError::ConnectionClose(e) => {
                            error!("The connection has been closed :", e.code);
                            error!("Error :", e.reason);
                            event_bus.send(WsMessage::ConnectionClosed);
                        }
                        WebSocketError::MessageSendError(e) => {
                            error!("Error while sending message", e.to_string());
                            event_bus.send(WsMessage::ErrorOnMessage(e.to_string()))
                        }
                        _ => error!("Unexpected error while communicating with distant ws"),
                    },
                }
            }
            debug!("WebSocket Closed");
        });

        Self { tx: in_tx }
    }

    pub async fn close(&mut self) {
        self.tx.close().await.unwrap();
    }
}
