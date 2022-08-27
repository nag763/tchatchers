use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Display)]
#[serde(rename_all = "camelCase")]
pub enum WsMessageType {
    Error,
    NotConnected,
    Reply,
    KeepAlive,
    Ping,
    Pong,
    Closed,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsMessage {
    pub message_type: WsMessageType,
    pub content: String,
}
