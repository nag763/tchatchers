use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum WsMessageType {
    #[default]
    Send,
    Receive,
    RetrieveMessages,
    MessagesRetrieved,
    Reconnected,
    Disconnected,
}

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, derivative::Derivative, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "camelCase")]
#[derivative(Default)]
pub struct WsMessage {
    pub message_type: WsMessageType,
    #[derivative(Default(value = "Uuid::new_v4()"))]
    pub uuid: Uuid,
    pub jwt: Option<String>,
    pub content: Option<String>,
    pub author: Option<String>,
    #[derivative(Default(value = "chrono::offset::Utc::now()"))]
    pub timestamp: DateTime<Utc>,
}
