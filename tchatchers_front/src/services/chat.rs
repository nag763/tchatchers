use crate::services::event_bus::{EventBus, Request};
use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use gloo_console::{debug, error};
use gloo_net::websocket::{futures::WebSocket, Message, WebSocketError};
use wasm_bindgen_futures::spawn_local;
use yew_agent::Dispatched;

#[derive(Clone, Debug)]
pub struct WebsocketService {
    pub tx: Sender<String>,
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
        let ws_addr = format!("{}//{}/ws/{}", ws_protocol, host, room);
        let ws = WebSocket::open(&ws_addr).unwrap();

        let (mut write, mut read) = ws.split();

        let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<String>(1000);
        let mut event_bus = EventBus::dispatcher();

        spawn_local(async move {
            while let Some(s) = in_rx.next().await {
                write.send(Message::Text(s)).await.unwrap();
            }
        });

        spawn_local(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(data)) => {
                        debug!("from websocket:", &data);
                        match data.as_str() {
                            "Pong" => event_bus.send(Request::EventBusKeepAlive),
                            _ => event_bus.send(Request::EventBusMsg(data)),
                        }
                    }
                    Ok(Message::Bytes(b)) => {
                        let decoded = std::str::from_utf8(&b);
                        if let Ok(val) = decoded {
                            debug!("from websocket: {}", val);
                            event_bus.send(Request::EventBusMsg(val.into()));
                        }
                    }
                    Err(e) => match e {
                        WebSocketError::ConnectionError => {
                            error!("Error on connection");
                            event_bus.send(Request::EventBusNotConnected);
                        }
                        WebSocketError::ConnectionClose(e) => {
                            error!("The connection has been closed :", e.code);
                            event_bus.send(Request::EventBusClosed);
                        }
                        WebSocketError::MessageSendError(e) => {
                            error!("Error while sending message", e.to_string());
                            event_bus.send(Request::EventBusErr(e.to_string()))
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
