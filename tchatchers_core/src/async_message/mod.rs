pub mod async_payload;
pub mod processor;

use chrono::Utc;
use derive_more::Display;
use redis::Commands;
use serde::{Deserialize, Serialize};

use self::async_payload::AsyncPayload;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Display)]
pub enum AsyncMessage {
    LoggedUsers(i32),
}

#[derive(Debug, Clone, Copy, Display)]
pub enum AsyncQueue {
    LoggedUsers,
}

impl AsyncQueue {
    pub(crate) fn delete(&self, list: Vec<AsyncPayload>, conn: &mut redis::Connection) -> usize {
        let id_list: Vec<String> = list.into_iter().filter_map(|li| li.id).collect();
        debug!("[{self}] IDs to delete : {id_list:#?}");
        conn.xdel(self.to_string(), &id_list).unwrap()
    }
}

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct AsyncOperationPGType {
    pub entity_id: i32,
    pub queue_id: String,
    pub timestamp: chrono::DateTime<Utc>,
}

impl AsyncMessage {
    fn get_queue(&self) -> AsyncQueue {
        match self {
            AsyncMessage::LoggedUsers(_) => AsyncQueue::LoggedUsers,
        }
    }

    pub fn spawn(self, conn: &mut redis::Connection) {
        let queue_name = self.get_queue().to_string();
        AsyncPayload::new(&queue_name, self).spawn(queue_name.as_str(), conn);
    }

    pub fn read_events(
        queue: AsyncQueue,
        conn: &mut redis::Connection,
    ) -> Option<Vec<AsyncPayload>> {
        AsyncPayload::read_events(&queue.to_string(), conn)
    }
}
