//! A WS message is a standard used to communicate between the client and the
//! server applications.
//!
//! It contains the essential data, and is persisted within rooms.

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use crate::user::PartialUser;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// The types of messages shared between users.
///
/// Some WS messages are containing data that have to be transmitted to everyone
/// in the room (ie Send) or actions to run either on client side
/// (MessagesRetrieved) or server side (RetrieveMessages)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum WsMessage {
    /// Content to be shared among all subscribers of the application.
    Send(WsMessageContent),
    /// Content to be displayed on client side.
    Receive(WsMessageContent),
    /// Action sent by a client to retrieve all messages of the room.
    ///
    /// Useful when a client connects to a chat that had messages before he
    /// joined.
    RetrieveMessages(Uuid),
    /// Information sent by the server to inform all the messages have been
    /// retrieved and sent to the client.
    MessagesRetrieved {
        messages: Vec<WsMessageContent>,
        session_id: Uuid,
    },
    MessageUpdated(Vec<WsMessageContent>),
    Pong,
    Ping,
    Close,
    ClientKeepAlive,
    /// Action to inform the server that the client reconnected.
    #[cfg(feature = "front")]
    ClientReconnected,
    #[cfg(feature = "front")]
    /// Action to inform the server that the client disconnected.
    ClientDisconnected,
    #[cfg(feature = "front")]
    ConnectionClosed,
    #[cfg(feature = "front")]
    ErrorOnMessage(String),
    Seen(Vec<WsMessageContent>),
}

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    derivative::Derivative,
    PartialEq,
    Eq,
    Hash,
    Default,
    Copy,
)]
pub enum WsReceptionStatus {
    #[default]
    NotSent,
    Sent,
    Seen,
}

/// Standard used to communicate inside WS between the client and the server
/// applications.
#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, derivative::Derivative, PartialEq, Eq, Hash,
)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct WsMessageContent {
    /// The message identifier, must be unique.
    #[derivative(Default(value = "Uuid::new_v4()"))]
    pub uuid: Uuid,
    /// The content of the message.
    pub content: String,
    /// The author of the message.
    ///
    /// Is empty when the message is emitted by the server.
    pub author: PartialUser,
    /// When the message has been emitted.
    #[derivative(Default(value = "chrono::offset::Utc::now()"))]
    pub timestamp: DateTime<Utc>,
    /// The room on which the message has been emitted.
    pub room: String,
    pub reception_status: WsReceptionStatus,
}
