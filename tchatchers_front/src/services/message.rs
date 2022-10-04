use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Display, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum WsBusMessageType {
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
pub struct WsBusMessage {
    pub message_type: WsBusMessageType,
    pub content: String,
}
