//! This module defines the `ReportedMessage` struct and its associated functionality.
//!
//! The `ReportedMessage` struct represents a reported message. It contains information such as the ID of the message,
//! the ID of the reporter, the UUID of the message, and the creation timestamp.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[cfg(feature = "back")]
use sqlx::{postgres::PgQueryResult, PgPool};
use uuid::Uuid;

/// Represents a reported message.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "back", derive(sqlx::FromRow))]
pub struct ReportedMessage {
    /// The base ID of the reported message.
    pub id: i32,
    /// The ID of the reporter associated with the message.
    pub reporter_id: i32,
    /// The UUID of the reported message.
    pub message_uuid: Uuid,
    /// The creation timestamp of the reported message.
    pub created_at: DateTime<Utc>,
}

impl ReportedMessage {
    /// Inserts a reported message into the database.
    ///
    /// # Arguments
    ///
    /// - `reporter_id`: The ID of the reporter associated with the message.
    /// - `message_uuid`: The UUID of the reported message.
    /// - `pool`: The PostgreSQL connection pool.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating whether the insertion was successful or not.
    #[cfg(feature = "back")]
    pub async fn insert(
        reporter_id: i32,
        message_uuid: Uuid,
        pool: &PgPool,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("INSERT INTO REPORTED_MESSAGE(reporter_id, message_uuid) VALUES ($1,$2)")
            .bind(reporter_id)
            .bind(message_uuid)
            .execute(pool)
            .await
    }
}
