use std::collections::HashMap;

use redis::{
    streams::{StreamReadOptions, StreamReadReply},
    Commands,
};
use serde::{Deserialize, Serialize};

use super::AsyncMessage;

lazy_static! {
    static ref DEFAULT_EVENT_OPTIONS: StreamReadOptions = StreamReadOptions::default().block(0);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncPayload {
    pub id: Option<String>,
    pub queue: String,
    pub entity: AsyncMessage,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, derive_more::Display, Debug)]
#[display(fmt = "[{kind}] {reason}")]
pub struct AsyncPayloadParseError {
    kind: AsyncPayloadParseErrorKind,
    reason: String,
}

#[derive(Clone, Copy, derive_more::Display, Debug)]
pub enum AsyncPayloadParseErrorKind {
    UnparseableRedisValue,
    UndeserializableValue,
    TimestampNotCorrect,
}

impl From<serde_json::Error> for AsyncPayloadParseError {
    fn from(value: serde_json::Error) -> Self {
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
        let redis_entity: String = redis::from_redis_value(&value["val"])?;
        let redis_timestamp: String = redis::from_redis_value(&value["timestamp"])?;
        Ok(AsyncPayload {
            queue: queue.to_string(),
            id: Some(key.to_string()),
            entity: serde_json::from_str(&redis_entity)?,
            timestamp: chrono::DateTime::parse_from_rfc2822(&redis_timestamp)?.into(),
        })
    }
}

impl From<AsyncPayload> for [(String, String); 2] {
    fn from(val: AsyncPayload) -> Self {
        [
            ("val".into(), serde_json::to_string(&val.entity).unwrap()),
            ("timestamp".into(), chrono::Utc::now().to_rfc2822()),
        ]
    }
}

impl AsyncPayload {
    pub(crate) fn new(queue_name: &str, entity: AsyncMessage) -> Self {
        Self {
            id: None,
            queue: queue_name.to_string(),
            entity,
            timestamp: chrono::Utc::now(),
        }
    }

    pub(crate) fn spawn(self, queue_name: &str, conn: &mut redis::Connection) {
        let redis_payload: [(String, String); 2] = self.into();
        redis::Cmd::xadd(queue_name, "*", &redis_payload).execute(conn);
        debug!("[{queue_name}] Event registered");
    }

    pub(crate) fn read_events(queue_name: &str, conn: &mut redis::Connection) -> Option<Vec<Self>> {
        let stream_events: Option<StreamReadReply> = conn
            .xread_options(&[queue_name], &["0"], &DEFAULT_EVENT_OPTIONS)
            .unwrap();
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
                            error!("[{queue_name}] An error happened while parsing the value of the event #{event_id} : {e}");
                            warn!("[{queue_name}] Element #{event_id} will be rejected");
                            rejects.push(event_id);
                        }
                    }
                }
            }
            debug!(
                "[{queue_name}] Events fetched from queue : {}",
                events.len()
            );
            trace!("[{queue_name}] Events : \n{:#?}", events);
            if !rejects.is_empty() {
                warn!("[{queue_name}] Reject list isn't empty, rejects will be deleted.");
                let events: i32 = conn.xdel(&[queue_name], &rejects).unwrap();
                info!("[{queue_name}] {events} rejects deleted.");
            }
            Some(events)
        } else {
            info!("[{queue_name}] No events found in queue, returning none");
            None
        }
    }
}
