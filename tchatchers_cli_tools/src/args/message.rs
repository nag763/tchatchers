use sqlx::types::Uuid;

#[derive(Debug, Clone, clap::Subcommand)]
pub enum MessageArgAction {
    #[command(about="Delete all the messages passed as arguments")]
    Delete { messages_uuid: Vec<Uuid> },
}
