use crate::services::event_bus::{EventBus, Request};
use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use gloo_console::{debug, error, log};
use reqwasm::websocket::{futures::WebSocket, Message};
use yew_agent::Dispatched;

use wasm_bindgen_futures::spawn_local;

#[derive(Clone)]
pub struct WebsocketService {
    pub tx: Sender<String>,
}

impl Default for WebsocketService {
    fn default() -> Self {
        Self::new()
    }
}

impl WebsocketService {
    pub fn new() -> Self {
        let ws = WebSocket::open("ws://127.0.0.1:8080").unwrap();

        let (mut write, mut read) = ws.split();

        let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<String>(1000);
        let mut event_bus = EventBus::dispatcher();

        spawn_local(async move {
            while let Some(s) = in_rx.next().await {
                log::debug!("got event from channel! {}", s);
                write.send(Message::Text(s)).await.unwrap();
            }
        });

        spawn_local(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(data)) => {
                        log!("from websocket:", &data);
                        event_bus.send(Request::EventBusMsg(data));
                    }
                    Ok(Message::Bytes(b)) => {
                        let decoded = std::str::from_utf8(&b);
                        if let Ok(val) = decoded {
                            debug!("from websocket: {}", val);
                        }
                    }
                    Err(_e) => {
                        error!("error on websocket");
                    }
                }
            }
            debug!("WebSocket Closed");
        });

        Self { tx: in_tx }
    }
}
