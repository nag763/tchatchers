pub mod async_payload;
pub mod processor;

use chrono::Utc;
use derive_more::Display;
use redis::{streams::StreamReadOptions, AsyncCommands};
use serde::{Deserialize, Serialize};

use self::async_payload::AsyncPayload;

lazy_static! {
    static ref DEFAULT_EVENT_OPTIONS: StreamReadOptions = StreamReadOptions::default().block(0);
    static ref NOT_BLOCKING_OPTIONS: StreamReadOptions = StreamReadOptions::default().block(1);
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Display)]
pub enum AsyncMessage {
    LoggedUser(i32),
    MessageSeen(uuid::Uuid),
}

#[derive(Debug, Clone, Copy, Display, Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum AsyncQueue {
    LoggedUsers,
    MessagesSeen,
}

impl AsyncQueue {
    pub async fn delete(&self, list: Vec<String>, conn: &mut redis::aio::Connection) -> usize {
        debug!("[{self}] IDs to delete : {list:#?}");
        conn.xdel(self.to_string(), &list).await.unwrap()
    }

    pub async fn clear_with_timeout(&self, conn: &mut redis::aio::Connection) -> usize {
        let events = Self::read_events_with_timeout(self, conn).await;
        if let Some(events) = events {
            let id_list: Vec<String> = events.into_iter().filter_map(|li| li.id).collect();
            self.delete(id_list, conn).await
        } else {
            warn!("[{self}] No events found while attempting to clear the queue");
            0
        }
    }

    pub async fn read_events_with_timeout(
        &self,
        conn: &mut redis::aio::Connection,
    ) -> Option<Vec<AsyncPayload>> {
        AsyncPayload::read_events(&self.to_string(), &NOT_BLOCKING_OPTIONS, conn).await
    }

    pub async fn read_events(
        &self,
        conn: &mut redis::aio::Connection,
    ) -> Option<Vec<AsyncPayload>> {
        AsyncPayload::read_events(&self.to_string(), &DEFAULT_EVENT_OPTIONS, conn).await
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::LoggedUsers, Self::MessagesSeen].into_iter()
    }
}

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct AsyncOperationPGType<T> {
    pub entity_id: T,
    pub queue_id: String,
    pub timestamp: chrono::DateTime<Utc>,
}

impl AsyncMessage {
    fn get_queue(&self) -> AsyncQueue {
        match self {
            AsyncMessage::LoggedUser(_) => AsyncQueue::LoggedUsers,
            AsyncMessage::MessageSeen(_) => AsyncQueue::MessagesSeen,
        }
    }

    pub async fn spawn(self, conn: &mut redis::aio::Connection) {
        let queue_name = self.get_queue().to_string();
        let _id = AsyncPayload::new(&queue_name, self)
            .spawn(queue_name.as_str(), conn)
            .await
            .unwrap();
    }
}
