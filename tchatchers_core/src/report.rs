/// This module defines the data structures and functions related to reporting.
/// 
/// It provides the `ReportKind` enum, which represents the different types of reports
/// (either for a message or a profile). The `Report` struct represents a reported user,
/// containing information such as the IDs of the reporter and reported user, the UUID
/// of the reported message (if applicable), the report kind, and the creation timestamp.
/// 
/// In the backend feature, the module also provides methods for inserting reports into
/// the database, such as `user()` for reporting a user and `message()` for reporting a message.
/// These methods interact with the PostgreSQL database using the `sqlx` crate.
/// 
/// This module is used for handling and processing user reports within the application.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[cfg(any(feature = "back", feature = "cli"))]
use sqlx::{postgres::PgQueryResult, PgPool};
use uuid::Uuid;

/// Represents the kind of report that can be made.
///
/// The `ReportKind` enum is used to differentiate between reporting a message and reporting a profile.
/// It has two variants:
/// - `Message`: Indicates that the report is for a message.
/// - `Profile`: Indicates that the report is for a profile.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(any(feature = "back", feature = "cli"), derive(sqlx::Type))]
#[repr(i32)]
pub enum ReportKind {
    Message = 1,
    Profile = 2,
}

/// Represents a report.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "back", derive(sqlx::FromRow))]
pub struct Report {
    /// The ID of the report.
    pub id: i32,
    /// The ID of the reporter.
    pub reporter_id: i32,
    /// The ID of the reported user.
    pub reported_id: Option<i32>,
    /// The UUID of the reported message.
    pub message_uuid: Option<Uuid>,
    /// The kind of report.
    #[cfg_attr(any(feature = "back", feature = "cli"), sqlx(rename = "report_kind_id"))]
    pub report_kind: ReportKind,
    /// The creation timestamp.
    pub created_at: DateTime<Utc>,
}

#[cfg(feature = "back")]
impl Report {
    
    /// Report a user.
    ///
    /// # Arguments
    ///
    /// - `reporter_id`: The ID of the reporter.
    /// - `reported_id`: The ID of the reported user.
    /// - `pool`: The database connection pool.
    ///
    /// This function inserts a new entry in the `REPORT` table of the database to report a user. It takes the ID of the reporter and the ID of the reported user as arguments. The `report_kind_id` field is set to `ReportKind::Profile`.
    ///
    pub async fn user(
        reporter_id: i32,
        reported_id: i32,
        pool: &PgPool,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("INSERT INTO REPORT(reporter_id, reported_id, report_kind_id) VALUES ($1, $2, $3)")
            .bind(reporter_id)
            .bind(reported_id)
            .bind(ReportKind::Profile)
            .execute(pool)
            .await
    }

    /// Report a message.
    ///
    /// # Arguments
    ///
    /// - `reporter_id`: The ID of the reporter.
    /// - `message_uuid`: The UUID of the reported message.
    /// - `pool`: The database connection pool.
    ///
    /// This function inserts a new entry in the `REPORT` table of the database to report a message. It takes the ID of the reporter and the UUID of the reported message as arguments. The `report_kind_id` field is set to `ReportKind::Message`.
    pub async fn message(
        reporter_id: i32,
        message_uuid: &Uuid,
        pool: &PgPool,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("INSERT INTO REPORT(reporter_id, message_uuid, report_kind_id) VALUES ($1, $2, $3)")
            .bind(reporter_id)
            .bind(message_uuid)
            .bind(ReportKind::Message)
            .execute(pool)
            .await
    }
}
