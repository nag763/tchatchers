use std::{collections::HashMap, future::Future, pin::Pin};

use sqlx::PgPool;

use crate::user::User;

use super::{async_payload::AsyncPayload, AsyncMessage, AsyncOperationPGType, AsyncQueue};

async fn process_logged_users(payloads: &Vec<AsyncPayload>, pool: &PgPool) {
    let mut entities_to_update: HashMap<i32, AsyncOperationPGType> =
        HashMap::with_capacity(payloads.capacity());
    for payload in payloads {
        let AsyncMessage::LoggedUsers(id) = payload.entity;

        if let Some(payload_id) = &payload.id {
            entities_to_update.insert(
                id,
                AsyncOperationPGType {
                    entity_id: id,
                    queue_id: payload_id.clone(),
                    timestamp: payload.timestamp,
                },
            );
        }
    }

    User::mark_users_as_logged(entities_to_update.into_values().collect(), pool)
        .await
        .unwrap();
}

fn get_processor<'a>(
    queue: AsyncQueue,
    payloads: &'a Vec<AsyncPayload>,
    pool: &'a PgPool,
) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
    match queue {
        AsyncQueue::LoggedUsers => return Box::pin(process_logged_users(payloads, pool)),
    }
}

pub async fn process(
    queue: AsyncQueue,
    messages: Vec<AsyncPayload>,
    pg_pool: &PgPool,
    redis_conn: &mut redis::aio::Connection,
) {
    let number_of_messages = messages.len();
    let processor = get_processor(queue, &messages, pg_pool);
    processor.await;
    info!("[{queue}] {number_of_messages} Messages passed");
    let number_of_id_deleted = queue.delete(messages, redis_conn).await;
    if number_of_id_deleted != number_of_messages {
        warn!("[{queue}] The number of ID deleted from the queue doesn't match the number of initial elements : Messages ({number_of_messages}) ; Deleted ({number_of_id_deleted})")
    } else {
        info!("[{queue}] Was successfully cleared of its events")
    }
}
