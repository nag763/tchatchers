use std::{future::Future, pin::Pin};

use sqlx::PgPool;

use crate::user::User;

use super::{async_payload::AsyncPayload, AsyncMessage, AsyncOperationPGType, AsyncQueue};

async fn process_logged_users(payloads: Vec<AsyncPayload>, pool: PgPool) -> () {
    let mut entities_to_update: Vec<AsyncOperationPGType> = Vec::with_capacity(payloads.capacity());
    for payload in payloads {
        let AsyncMessage::LoggedUsers(id) = payload.entity;
        if let Some(payload_id) = payload.id {
            entities_to_update.push(AsyncOperationPGType {
                entity_id: id,
                queue_id: payload_id,
            })
        }
    }
    let _ = User::mark_users_as_logged(entities_to_update, &pool).await;
}

fn get_processor(
    queue: AsyncQueue,
    payloads: Vec<AsyncPayload>,
    pool: PgPool,
) -> Pin<Box<dyn Future<Output = ()>>> {
    match queue {
        AsyncQueue::LoggedUsers => return Box::pin(process_logged_users(payloads, pool)),
    }
}

pub async fn process(queue: AsyncQueue, messages: Vec<AsyncPayload>, pool: PgPool) {
    let processor = get_processor(queue, messages, pool);
    processor.await;
}
