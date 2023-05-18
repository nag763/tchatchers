//! This module contains the definition of the `ReportedUser` struct and its associated methods.
//!
//! The `ReportedUser` struct represents a reported user and includes fields such as ID, reporter ID, reported ID, and creation timestamp. It is used to store information about users who have been reported.
//!
//! The module also provides an implementation block for the `ReportedUser` struct, which includes a method for inserting a reported user into the database when the `back` feature is enabled.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[cfg(feature = "back")]
use sqlx::{postgres::PgQueryResult, PgPool};

/// Represents a reported user.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "back", derive(sqlx::FromRow))]
pub struct ReportedUser {
    /// The ID of the reported user.
    pub id: i32,
    /// The ID of the reporter.
    pub reporter_id: i32,
    /// The ID of the reported user.
    pub reported_id: i32,

    /// The creation timestamp.
    pub created_at: DateTime<Utc>,
}

impl ReportedUser {
    #[cfg(feature = "back")]
    /// Inserts a reported user into the database.
    ///
    /// # Arguments
    ///
    /// * `reporter_id` - The ID of the reporter.
    /// * `reported_id` - The ID of the reported user.
    /// * `pool` - The PostgreSQL connection pool.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating the success or failure of the operation.
    pub async fn insert(
        reporter_id: i32,
        reported_id: i32,
        pool: &PgPool,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("INSERT INTO REPORTED_USER(reporter_id, reported_id) VALUES ($1,$2)")
            .bind(reporter_id)
            .bind(reported_id)
            .execute(pool)
            .await
    }
}
