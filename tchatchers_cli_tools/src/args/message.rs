use sqlx::types::Uuid;

/// The actions that can be run on the messages stored in the database.
#[derive(Debug, Clone, clap::Subcommand)]
pub enum MessageArgAction {
    /// Delete all the messages passed as arguments.
    #[command(about = "Delete all the messages passed as arguments")]
    Delete {
        /// UUIDs of the messages to delete.
        messages_uuid: Vec<Uuid>,
    },
}
