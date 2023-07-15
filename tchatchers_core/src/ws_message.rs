//! A WS message is a standard used to communicate between the client and the
//! server applications.
//!
//! It contains the essential data, and is persisted within rooms.

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

#[cfg(any(feature = "back", feature = "cli", feature = "async"))]
use crate::async_message::{AsyncOperationPGType, AsyncQueue};
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
    /// Indicates that the user has seeen the messages.
    MessagesSeen(Vec<Uuid>),
    /// Responds to Ping !
    Pong,
    /// Service !
    Ping,
    /// Make aware that one of the party wants to close the connection.
    Close,
    /// Keep alive to not close the connection.
    ClientKeepAlive,
    /// Action to inform the server that the client reconnected.
    #[cfg(feature = "front")]
    ClientReconnected,
    #[cfg(feature = "front")]
    /// Action to inform the server that the client disconnected.
    ClientDisconnected,
    /// Message to inform the connection will be closed by the client.
    #[cfg(feature = "front")]
    ConnectionClosed,
    /// Inform that there is an error on the incoming message.
    #[cfg(feature = "front")]
    ErrorOnMessage(String),
    /// Inform that one has seen the messages.
    Seen(Vec<Uuid>),
    /// Deletes a message
    Delete(Uuid),
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
#[cfg_attr(
    any(feature = "back", feature = "cli", feature = "async"),
    derive(sqlx::Type)
)]
#[repr(i32)]
pub enum WsReceptionStatus {
    #[default]
    NotSent = 1,
    Sent = 2,
    Seen = 3,
}

/// Standard used to communicate inside WS between the client and the server
/// applications.
#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, derivative::Derivative, PartialEq, Eq, Hash,
)]
#[derivative(Default)]
#[cfg_attr(
    any(feature = "back", feature = "cli", feature = "async"),
    derive(sqlx::FromRow)
)]
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
    #[cfg_attr(
        any(feature = "back", feature = "cli", feature = "async"),
        sqlx(flatten)
    )]
    pub author: PartialUser,
    /// When the message has been emitted.
    #[derivative(Default(value = "chrono::offset::Utc::now()"))]
    pub timestamp: DateTime<Utc>,
    /// The room on which the message has been emitted.
    pub room: String,
    /// Whether a message has been received or not.
    pub reception_status: WsReceptionStatus,
}

#[cfg(any(feature = "back", feature = "cli", feature = "async"))]
impl WsMessageContent {
    /// Get one message from the database.
    ///
    /// # Arguments
    ///
    /// - uuid: message id.
    pub async fn get_one(uuid: &Uuid, pool: &sqlx::PgPool) -> Option<Self> {
        sqlx::query_as(
            "SELECT * FROM MESSAGE m INNER JOIN CHATTER c ON m.author = c.id WHERE uuid=$1 LIMIT 1",
        )
        .bind(uuid)
        .fetch_optional(pool)
        .await
        .unwrap()
    }

    /// Returns the first 100 messages for a given room name.
    ///
    /// # Arguments
    ///
    /// - room_name : The room the query is made for.
    pub async fn query_all_for_room(room_name: &str, pool: &sqlx::PgPool) -> Vec<Self> {
        sqlx::query_as("SELECT * FROM MESSAGE m INNER JOIN CHATTER c ON m.author = c.id WHERE room=$1 ORDER BY timestamp DESC LIMIT 100 ")
            .bind(room_name)
            .fetch_all(pool)
            .await
            .unwrap()
    }

    /// Insert the message in the database.
    ///
    /// # Arguments
    ///
    /// - pool : the connection pool.
    pub async fn persist(
        &self,
        pool: &sqlx::PgPool,
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        sqlx::query("INSERT INTO MESSAGE(uuid, content, author, timestamp, room, reception_status) VALUES ($1,$2,$3,$4,$5,$6)")
            .bind(self.uuid)
            .bind(&self.content)
            .bind(self.author.id)
            .bind(self.timestamp)
            .bind(&self.room)
            .bind(self.reception_status)
            .execute(pool)
            .await
    }

    /// Mark a list of existing messages as seen?
    ///
    /// # Arguments
    ///
    /// - messages_uuid : the list of messages seen.
    /// - pool : the connection pool.
    pub async fn mark_as_seen(
        messages_uuid: &Vec<Uuid>,
        pool: &sqlx::PgPool,
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        sqlx::query("UPDATE MESSAGE SET reception_status=$1 WHERE uuid = ANY($2)")
            .bind(WsReceptionStatus::Seen)
            .bind(messages_uuid)
            .execute(pool)
            .await
    }

    /// Mark a list of existing messages as seen?
    ///
    /// # Arguments
    ///
    /// - messages_uuid : the list of messages seen.
    /// - pool : the connection pool.
    pub async fn mark_as_seen_async(
        messages: Vec<AsyncOperationPGType<Uuid>>,
        pool: &sqlx::PgPool,
    ) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        let updated_records: u64 =
            sqlx::query("UPDATE MESSAGE m SET RECEPTION_STATUS = $1 WHERE uuid = ANY($2)")
                .bind(WsReceptionStatus::Seen)
                .bind(messages.iter().map(|m| m.entity_id).collect::<Vec<Uuid>>())
                .execute(&mut tx)
                .await?
                .rows_affected();

        let failed_records = messages.len() - updated_records as usize;

        sqlx::query(
            "
        INSERT INTO PROCESS_REPORT(process_id, successfull_records, failed_records) 
        VALUES($1, $2, $3)
        ",
        )
        .bind(AsyncQueue::MessagesSeen)
        .bind(updated_records as i64)
        .bind(failed_records as i64)
        .execute(&mut tx)
        .await
        .unwrap();

        tx.commit().await.unwrap();

        Ok(())
    }

    /// Deletes the message present in a room.
    ///
    /// # Arguments
    /// - room_name : The room where the messages needs to be deleted.
    /// - pool : The connection pool.
    pub async fn delete_message_in_room(
        room_name: &str,
        pool: &sqlx::PgPool,
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        sqlx::query("DELETE FROM MESSAGE WHERE room = $1")
            .bind(room_name)
            .execute(pool)
            .await
    }

    pub async fn delete_messages(
        messages_uuid: &Vec<Uuid>,
        pool: &sqlx::PgPool,
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        sqlx::query("DELETE FROM MESSAGE WHERE uuid = ANY($1)")
            .bind(messages_uuid)
            .execute(pool)
            .await
    }
}

/// A struct used for the activity reports.
#[derive(Debug)]
#[cfg_attr(any(feature = "back", feature = "cli"), derive(sqlx::FromRow))]
pub struct WsMessageStats {
    /// The number of messages in the room.
    pub number_of_messages: i64,
    /// The room name.
    pub room: String,
}

#[cfg(feature = "cli")]
impl WsMessageStats {
    /// Returns the activity from the database.
    ///
    /// # Arguments
    ///
    /// - pool : The connection pool.
    pub async fn get_activity(pool: &sqlx::PgPool) -> Vec<Self> {
        sqlx::query_as("SELECT COUNT(*) as number_of_messages, room FROM MESSAGE GROUP BY room ORDER BY number_of_messages DESC")
            .fetch_all(pool)
            .await
            .unwrap()
    }
}
