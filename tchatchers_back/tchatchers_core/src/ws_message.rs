use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum WsMessageType {
    #[default]
    Send,
    Receive,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, derivative::Derivative)]
#[serde(rename_all = "camelCase")]
#[derivative(Default)]
pub struct WsMessage {
    pub message_type: WsMessageType,
    #[derivative(Default(value = "Uuid::new_v4()"))]
    pub uuid: Uuid,
    pub jwt: Option<String>,
    pub content: Option<String>,
    pub author: Option<String>,
}
