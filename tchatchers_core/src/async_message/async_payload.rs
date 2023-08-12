//! This module provides functionalities for handling asynchronous payloads used in the core crate.
//!
//! It includes a struct for representing an asynchronous payload (`AsyncPayload`) and related error types.
//! The `AsyncPayload` struct contains information about the payload, such as the ID, queue name, entity data,
//! and timestamp. It also provides methods for creating, spawning, and reading events from Redis streams.

use std::collections::HashMap;

use redis::{
    streams::{StreamReadOptions, StreamReadReply},
    AsyncCommands, RedisError,
};
use serde::{Deserialize, Serialize};

use super::AsyncMessage;

/// Represents an asynchronous payload that is stored in Redis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncPayload {
    /// The ID of the payload.
    pub id: Option<String>,
    /// The name of the queue the payload belongs to.
    pub queue: String,
    /// The entity data of the payload.
    pub entity: AsyncMessage,
    /// The timestamp of when the payload was created.
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Represents an error that occurs during the parsing of an `AsyncPayload`.
#[derive(Clone, derive_more::Display, Debug)]
#[display(fmt = "[{kind}] {reason}")]
pub struct AsyncPayloadParseError {
    /// The kind of parse error.
    kind: AsyncPayloadParseErrorKind,
    /// The reason for the parse error.
    reason: String,
}

/// Represents the different kinds of errors that can occur during parsing of an `AsyncPayload`.
#[derive(Clone, Copy, derive_more::Display, Debug)]
pub enum AsyncPayloadParseErrorKind {
    /// The Redis value could not be parsed.
    UnparseableRedisValue,
    /// The value could not be deserialized.
    UndeserializableValue,
    /// The timestamp format is not correct.
    TimestampNotCorrect,
}

impl From<postcard::Error> for AsyncPayloadParseError {
    fn from(value: postcard::Error) -> Self {
        Self {
            kind: AsyncPayloadParseErrorKind::UndeserializableValue,
            reason: value.to_string(),
        }
    }
}

impl From<redis::RedisError> for AsyncPayloadParseError {
    fn from(value: redis::RedisError) -> Self {
        Self {
            kind: AsyncPayloadParseErrorKind::UnparseableRedisValue,
            reason: value.to_string(),
        }
    }
}

impl From<chrono::ParseError> for AsyncPayloadParseError {
    fn from(value: chrono::ParseError) -> Self {
        Self {
            kind: AsyncPayloadParseErrorKind::TimestampNotCorrect,
            reason: value.to_string(),
        }
    }
}

impl TryFrom<(&str, &str, HashMap<String, redis::Value>)> for AsyncPayload {
    type Error = AsyncPayloadParseError;

    fn try_from(map: (&str, &str, HashMap<String, redis::Value>)) -> Result<Self, Self::Error> {
        let queue = map.0;
        let key = map.1;
        let value = map.2;
        let redis_entity: Vec<u8> = redis::from_redis_value(&value["val"])?;
        let redis_timestamp: String = redis::from_redis_value(&value["timestamp"])?;
        Ok(AsyncPayload {
            queue: queue.to_string(),
            id: Some(key.to_string()),
            entity: postcard::from_bytes(&redis_entity)?,
            timestamp: chrono::DateTime::parse_from_rfc2822(&redis_timestamp)?.into(),
        })
    }
}

impl From<AsyncPayload> for [(String, Vec<u8>); 2] {
    fn from(val: AsyncPayload) -> Self {
        [
            ("val".into(), postcard::to_stdvec(&val.entity).unwrap()),
            (
                "timestamp".into(),
                chrono::Utc::now().to_rfc2822().as_bytes().into(),
            ),
        ]
    }
}

impl AsyncPayload {
    /// Creates a new `AsyncPayload` with the given queue name and entity data.
    pub(crate) fn new(queue_name: &str, entity: AsyncMessage) -> Self {
        Self {
            id: None,
            queue: queue_name.to_string(),
            entity,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Spawns the `AsyncPayload` to the specified queue and returns the event ID.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue to spawn the payload into.
    /// * `conn` - A mutable reference to the Redis connection.
    ///
    /// # Returns
    ///
    /// A `Result` containing the event ID if successful, or a `RedisError` if an error occurs.
    pub(crate) async fn spawn(
        self,
        queue_name: &str,
        conn: &mut redis::aio::Connection,
    ) -> Result<String, RedisError> {
        let redis_payload: [(String, Vec<u8>); 2] = self.into();
        let event_id: String = conn.xadd(queue_name, "*", &redis_payload).await?;
        debug!("[{queue_name}] Event #{event_id} registered");
        Ok(event_id)
    }

    /// Reads events from the specified queue with the given options.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue to read events from.
    /// * `options` - The options to customize the event reading behavior.
    /// * `conn` - A mutable reference to the Redis connection.
    ///
    /// # Returns
    ///
    /// An `Option` containing a `Vec` of `AsyncPayload` if events are found, or `None` if no events are found.
    pub(crate) async fn read_events(
        queue_name: &str,
        options: &StreamReadOptions,
        conn: &mut redis::aio::Connection,
    ) -> Result<Option<Vec<Self>>, redis::RedisError> {
        let stream_events: Option<StreamReadReply> =
            conn.xread_options(&[queue_name], &["0"], options).await?;
        if let Some(stream_events) = stream_events {
            let mut events: Vec<Self> = Vec::new();
            let mut rejects: Vec<String> = Vec::new();
            for keys in stream_events.keys {
                for id in keys.ids {
                    let event_id = id.id;
                    let map = id.map;
                    match Self::try_from((queue_name, event_id.as_str(), map)) {
                        Ok(event) => events.push(event),
                        Err(e) => {
                            error!("[{queue_name}] An error happened while parsing the value of the event #{event_id}: {e}");
                            warn!("[{queue_name}] Element #{event_id} will be rejected");
                            rejects.push(event_id);
                        }
                    }
                }
            }
            debug!("[{queue_name}] Events fetched from queue: {}", events.len());
            trace!("[{queue_name}] Events:\n{:#?}", events);
            if !rejects.is_empty() {
                warn!("[{queue_name}] Reject list isn't empty, rejects will be deleted.");
                let events: i32 = conn.xdel(&[queue_name], &rejects).await?;
                info!("[{queue_name}] {events} rejects deleted.");
            }
            Ok(Some(events))
        } else {
            info!("[{queue_name}] No events found in queue, returning none");
            Ok(None)
        }
    }
}
