use sqlx::types::Uuid;
use tchatchers_core::ws_message::WsMessageContent;

use crate::errors::CliError;

pub struct MessageAction;

impl MessageAction {
    /// Deletes messages with the given UUIDs from the database.
    ///
    /// # Arguments
    ///
    /// * `messages_uuid` - A vector of UUIDs representing the messages to delete.
    ///
    /// # Returns
    ///
    /// * `Result<(), CliError>` - Returns `Ok(())` if the operation was successful, otherwise
    /// returns an error of type `CliError`.
    pub async fn delete_messages(messages_uuid: Vec<Uuid>) -> Result<(), CliError> {
        let pool = tchatchers_core::pool::get_pg_pool().await;
        WsMessageContent::delete_messages(&messages_uuid, &pool).await?;
        Ok(())
    }
}
