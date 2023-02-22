use sqlx::types::Uuid;
use tchatchers_core::ws_message::WsMessageContent;

use crate::errors::CliError;

pub struct MessageAction;

impl MessageAction {
    pub async fn delete_messages(messages_uuid: Vec<Uuid>) -> Result<(), CliError> {
        let pool = tchatchers_core::pool::get_pg_pool().await;
        WsMessageContent::delete_messages(&messages_uuid, &pool).await?;
        Ok(())
    }
}
