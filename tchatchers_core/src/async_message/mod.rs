pub mod async_payload;
pub mod processor;

use derive_more::Display;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgHasArrayType;

use self::async_payload::AsyncPayload;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Display)]
pub enum AsyncMessage {
    LoggedUsers(i32),
}

#[derive(Debug, Clone, Copy, Display)]
pub enum AsyncQueue {
    LoggedUsers,
}

#[derive(sqlx::Type, sqlx::FromRow, Clone, Debug)]
pub struct AsyncOperationPGType {
    entity_id: i32,
    queue_id: String,
}

impl PgHasArrayType for AsyncOperationPGType {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("chatter_logged_on")
    }
}

impl AsyncMessage {
    fn get_queue(&self) -> AsyncQueue {
        match self {
            AsyncMessage::LoggedUsers(_) => AsyncQueue::LoggedUsers,
        }
    }

    pub fn spawn(self, conn: &mut redis::Connection) {
        let queue_name = self.get_queue().to_string();
        AsyncPayload::new(&queue_name, self).spawn(&queue_name.as_str(), conn);
    }

    pub fn read_events(
        queue: AsyncQueue,
        conn: &mut redis::Connection,
    ) -> Option<Vec<AsyncPayload>> {
        AsyncPayload::read_events(&queue.to_string(), conn)
    }
}
