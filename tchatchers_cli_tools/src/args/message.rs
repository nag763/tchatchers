use sqlx::types::Uuid;

#[derive(Debug, Clone, clap::Subcommand)]
pub enum MessageArgAction {
    Delete { messages_uuid: Vec<Uuid> },
}
