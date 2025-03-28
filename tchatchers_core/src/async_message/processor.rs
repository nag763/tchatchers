//! This module provides functionality for processing asynchronous messages in different queues.
//!
//! It includes functions for processing specific types of messages, such as logged users and seen messages.
//! The `process` function is the main entry point, which takes a queue, a list of messages, a PostgreSQL pool,
//! and a Redis connection. It delegates the processing to the appropriate function based on the queue type,
//! updates the corresponding entities in the database, and clears the processed messages from the queue.

use std::{
    collections::{HashMap, HashSet},
    future::Future,
    pin::Pin,
};

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    user::{PartialUser, User},
    ws_message::WsMessageContent,
};

use super::{async_payload::AsyncPayload, AsyncMessage, AsyncOperationPGType, AsyncQueue};

/// Processes messages related to logged users.
///
/// This function takes a vector of `AsyncPayload` messages and a PostgreSQL pool.
/// It extracts the logged user IDs from the messages and updates the corresponding entities
/// in the database as "logged". Any messages that don't match the expected format are skipped.
///
/// # Arguments
///
/// * `payloads` - A vector of `AsyncPayload` messages to process.
/// * `pool` - A reference to the PostgreSQL pool for database operations.
async fn process_logged_users(
    payloads: &Vec<AsyncPayload>,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    let mut entities_to_update: HashMap<i32, AsyncOperationPGType<i32>> =
        HashMap::with_capacity(payloads.capacity());
    for payload in payloads {
        let AsyncMessage::LoggedUser(id) = payload.entity else {
            warn!("Entity {:?} isn't matching the expected format", payload.id);
            continue;
        };

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

    User::mark_users_as_logged(entities_to_update.into_values().collect(), pool).await
}

/// Processes messages related to seen messages.
///
/// This function takes a vector of `AsyncPayload` messages and a PostgreSQL pool.
/// It extracts the seen message UUIDs from the messages and updates the corresponding entities
/// in the database as "seen". Any messages that don't match the expected format are skipped.
///
/// # Arguments
///
/// * `payloads` - A vector of `AsyncPayload` messages to process.
/// * `pool` - A reference to the PostgreSQL pool for database operations.
async fn messages_seen(payloads: &Vec<AsyncPayload>, pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut entities_to_update: HashMap<Uuid, AsyncOperationPGType<Uuid>> =
        HashMap::with_capacity(payloads.capacity());

    for payload in payloads {
        let AsyncMessage::MessageSeen(id) = payload.entity else {
            warn!("Entity {:?} isn't matching the expected format", payload.id);
            continue;
        };

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

    WsMessageContent::mark_as_seen_async(entities_to_update.into_values().collect(), pool).await
}

async fn persist_messages(payloads: &Vec<AsyncPayload>, pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut entities_to_update: HashMap<Uuid, WsMessageContent> =
        HashMap::with_capacity(payloads.capacity());

    for payload in payloads {
        let AsyncMessage::PersistMessage(entity) = payload.clone().entity else {
            warn!("Entity {:?} isn't matching the expected format", payload.id);
            continue;
        };

        entities_to_update.insert(entity.uuid, entity);
    }

    WsMessageContent::persist_async(entities_to_update.into_values().collect(), pool).await
}

async fn clean_rooms(payloads: &Vec<AsyncPayload>, pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut entities_to_delete: HashSet<&str> = HashSet::with_capacity(payloads.capacity());

    for payload in payloads {
        let AsyncMessage::CleanRoom(entity) = &payload.entity else {
            warn!("Entity {:?} isn't matching the expected format", payload.id);
            continue;
        };

        entities_to_delete.insert(entity);
    }

    WsMessageContent::clean_rooms(entities_to_delete, pool).await
}
async fn clear_user_data(payloads: &Vec<AsyncPayload>, pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut entities_to_clear: HashSet<&PartialUser> = HashSet::with_capacity(payloads.capacity());

    for payload in payloads {
        let AsyncMessage::RemoveUserData(entity) = &payload.entity else {
            warn!("Entity {:?} isn't matching the expected format", payload.id);
            continue;
        };

        entities_to_clear.insert(entity);
    }
    PartialUser::clear_data(entities_to_clear, pool).await
}

/// Returns the appropriate processor for the given queue.
///
/// This function takes a queue, a vector of `AsyncPayload` messages, and a PostgreSQL pool,
/// and returns a boxed future representing the appropriate processing function for the given queue.
/// The processing function will be executed asynchronously.
///
/// # Arguments
///
/// * `queue` - The queue type indicating the type of messages to process.
/// * `payloads` - A reference to the vector of `AsyncPayload` messages to process.
/// * `pool` - A reference to the PostgreSQL pool for database operations.
fn get_processor<'a>(
    queue: AsyncQueue,
    payloads: &'a Vec<AsyncPayload>,
    pool: &'a PgPool,
) -> Pin<Box<dyn Future<Output = Result<(), sqlx::Error>> + Send + 'a>> {
    match queue {
        AsyncQueue::LoggedUsers => Box::pin(process_logged_users(payloads, pool)),
        AsyncQueue::MessagesSeen => Box::pin(messages_seen(payloads, pool)),
        AsyncQueue::PersistMessage => Box::pin(persist_messages(payloads, pool)),
        AsyncQueue::CleanRoom => Box::pin(clean_rooms(payloads, pool)),
        AsyncQueue::RemoveUserData => Box::pin(clear_user_data(payloads, pool)),
    }
}

/// Processes messages in the specified queue.
///
/// This function takes a queue, a list of messages, a PostgreSQL pool, and a Redis connection.
/// It delegates the processing to the appropriate function based on the queue type, updates
/// the corresponding entities in the database, and clears the processed messages from the queue.
///
/// # Arguments
///
/// * `queue` - The queue type indicating the type of messages to process.
/// * `messages` - A vector of `AsyncPayload` messages to process.
/// * `pg_pool` - A reference to the PostgreSQL pool for database operations.
/// * `redis_conn` - A mutable reference to the Redis connection for queue operations.
pub async fn process(
    queue: AsyncQueue,
    messages: Vec<AsyncPayload>,
    pg_pool: &PgPool,
    redis_conn: &mut redis::aio::MultiplexedConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let number_of_messages = messages.len();
    let processor = get_processor(queue, &messages, pg_pool);
    processor.await?;
    info!("[{queue}] {number_of_messages} messages passed");
    let id_list: Vec<String> = messages.into_iter().filter_map(|li| li.id).collect();
    debug!("[{queue}] Deleting old messages...");
    let number_of_id_deleted = queue.delete(id_list, redis_conn).await?;
    if number_of_id_deleted != number_of_messages {
        warn!("[{queue}] The number of ID deleted from the queue doesn't match the number of initial elements : Messages ({number_of_messages}) ; Deleted ({number_of_id_deleted})");
    } else {
        info!("[{queue}] Was successfully cleared of its events");
    }
    Ok(())
}
