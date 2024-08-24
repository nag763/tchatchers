//! This module provides functionality related to asynchronous messages and processing queues.
//!
//! It includes sub-modules for working with asynchronous payloads and message processing.
//! The module also defines types for different types of asynchronous messages and queues,
//! as well as utility functions for interacting with the Redis stream and PostgreSQL database.

use chrono::Utc;
use derive_more::Display;
use redis::{streams::StreamReadOptions, AsyncCommands};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub mod async_payload;
pub mod processor;

use crate::{user::PartialUser, ws_message::WsMessageContent};

use self::async_payload::AsyncPayload;

lazy_static! {
    static ref DEFAULT_EVENT_OPTIONS: StreamReadOptions = StreamReadOptions::default().block(0);
    static ref NOT_BLOCKING_OPTIONS: StreamReadOptions = StreamReadOptions::default().block(1);
}

/// Represents different types of asynchronous messages.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
pub enum AsyncMessage {
    LoggedUser(i32),
    MessageSeen(uuid::Uuid),
    PersistMessage(WsMessageContent),
    CleanRoom(String),
    RemoveUserData(PartialUser),
}

/// Represents a queue report containing information about the latest executed processes for a queue.
#[derive(Debug, Clone, sqlx::FromRow, Display)]
#[display("[{process_id}#{id}] ({successfull_records}/{records_processed}) on {passed_at}")]
pub struct QueueReport {
    pub id: i32,
    pub process_id: AsyncQueue,
    pub failed_records: i32,
    pub successfull_records: i32,
    pub records_processed: i32,
    pub passed_at: chrono::DateTime<Utc>,
}

impl QueueReport {
    /// Retrieves the latest queue reports for the specified queue.
    ///
    /// # Arguments
    ///
    /// * `queue` - The queue for which to retrieve the latest reports.
    /// * `limit` - The maximum number of reports to retrieve. If `None`, retrieves only the latest report.
    /// * `pool` - A reference to the PostgreSQL pool for database operations.
    pub async fn latest_for_queue(
        queue: AsyncQueue,
        limit: Option<i64>,
        pool: &PgPool,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as(
            "SELECT * FROM PROCESS_REPORT WHERE process_id = $1 ORDER BY ID DESC LIMIT $2",
        )
        .bind(queue as i32)
        .bind(limit.unwrap_or(1))
        .fetch_all(pool)
        .await
    }
}

/// Represents different types of asynchronous queues.
#[derive(Debug, Clone, Copy, Display, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
#[repr(i32)]
pub enum AsyncQueue {
    LoggedUsers = 1,
    MessagesSeen = 2,
    PersistMessage = 3,
    CleanRoom = 4,
    RemoveUserData = 5,
}

impl AsyncQueue {
    /// Deletes the specified IDs from the queue.
    ///
    /// # Arguments
    ///
    /// * `list` - A vector of IDs to delete.
    /// * `conn` - A mutable reference to the Redis connection for queue operations.
    ///
    /// # Returns
    ///
    /// The number of IDs deleted from the queue.
    pub async fn delete(
        &self,
        list: Vec<String>,
        conn: &mut redis::aio::MultiplexedConnection,
    ) -> Result<usize, redis::RedisError> {
        debug!("[{self}] IDs to delete: {list:#?}");
        let res = conn.xdel(self.to_string(), &list).await?;
        debug!("[{self}] IDS deleted");
        Ok(res)
    }

    /// Clears the queue by deleting all events.
    ///
    /// This method reads events with a timeout and deletes them from the queue,
    /// effectively clearing the queue of its events.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the Redis connection for queue operations.
    ///
    /// # Returns
    ///
    /// The number of events cleared from the queue.
    pub async fn clear_with_timeout(
        &self,
        conn: &mut redis::aio::MultiplexedConnection,
    ) -> Result<usize, redis::RedisError> {
        let events = self.read_events_with_timeout(conn).await?;
        if let Some(events) = events {
            let id_list: Vec<String> = events.into_iter().filter_map(|li| li.id).collect();
            Ok(self.delete(id_list, conn).await?)
        } else {
            warn!("[{self}] No events found while attempting to clear the queue");
            Ok(0)
        }
    }

    /// Reads events from the queue with a blocking timeout.
    ///
    /// This method reads events from the queue with a blocking timeout using the specified Redis connection.
    /// It returns the events as a vector of `AsyncPayload` instances, or `None` if no events are found within the timeout.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the Redis connection for queue operations.
    ///
    /// # Returns
    ///
    /// A vector of `AsyncPayload` instances representing the events read from the queue, or `None` if no events are found.
    pub async fn read_events_with_timeout(
        &self,
        conn: &mut redis::aio::MultiplexedConnection,
    ) -> Result<Option<Vec<AsyncPayload>>, redis::RedisError> {
        AsyncPayload::read_events(&self.to_string(), &NOT_BLOCKING_OPTIONS, conn).await
    }

    /// Reads events from the queue.
    ///
    /// This method reads events from the queue using the specified Redis connection.
    /// It returns the events as a vector of `AsyncPayload` instances, or `None` if no events are found.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the Redis connection for queue operations.
    ///
    /// # Returns
    ///
    /// A vector of `AsyncPayload` instances representing the events read from the queue, or `None` if no events are found.
    pub async fn read_events(
        &self,
        conn: &mut redis::aio::MultiplexedConnection,
    ) -> Result<Option<Vec<AsyncPayload>>, redis::RedisError> {
        AsyncPayload::read_events(&self.to_string(), &DEFAULT_EVENT_OPTIONS, conn).await
    }

    /// Returns an iterator over all the async queue types.
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::LoggedUsers,
            Self::MessagesSeen,
            Self::PersistMessage,
            Self::CleanRoom,
            Self::RemoveUserData,
        ]
        .iter()
        .cloned()
    }
}

/// Represents an asynchronous operation in PostgreSQL.
#[derive(sqlx::FromRow, Clone, Debug)]
pub struct AsyncOperationPGType<T> {
    pub entity_id: T,
    pub queue_id: String,
    pub timestamp: chrono::DateTime<Utc>,
}

impl AsyncMessage {
    /// Retrieves the queue associated with the asynchronous message.
    fn get_queue(&self) -> AsyncQueue {
        match self {
            AsyncMessage::LoggedUser(_) => AsyncQueue::LoggedUsers,
            AsyncMessage::MessageSeen(_) => AsyncQueue::MessagesSeen,
            AsyncMessage::PersistMessage(_) => AsyncQueue::PersistMessage,
            AsyncMessage::CleanRoom(_) => AsyncQueue::CleanRoom,
            AsyncMessage::RemoveUserData(_) => AsyncQueue::RemoveUserData,
        }
    }

    /// Spawns the asynchronous message.
    ///
    /// This method creates a new `AsyncPayload` with the message and queues it for processing.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the Redis connection for queue operations.
    pub async fn spawn(self, conn: &mut redis::aio::MultiplexedConnection) {
        let queue_name = self.get_queue().to_string();
        let _id = AsyncPayload::new(&queue_name, self)
            .spawn(queue_name.as_str(), conn)
            .await
            .unwrap();
    }
}
