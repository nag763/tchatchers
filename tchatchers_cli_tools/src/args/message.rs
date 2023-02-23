use sqlx::types::Uuid;

/// The actions that can be run on the database.
#[derive(Debug, Clone, clap::Subcommand)]
pub enum MessageArgAction {
    #[command(about = "Delete all the messages passed as arguments")]
    Delete { messages_uuid: Vec<Uuid> },
}
