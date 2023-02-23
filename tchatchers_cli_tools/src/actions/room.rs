use tchatchers_core::ws_message::{WsMessageContent, WsMessageStats};

use crate::errors::CliError;

pub struct RoomAction;

impl RoomAction {
    pub async fn delete_messages(room_name: &str) -> Result<(), CliError> {
        let pool = tchatchers_core::pool::get_pg_pool().await;
        let result = WsMessageContent::delete_message_in_room(room_name, &pool).await?;
        println!(
            "{} messages deleted in room '{}'",
            result.rows_affected(),
            room_name
        );
        Ok(())
    }

    pub async fn get_messages(room_name: &str) -> Result<(), CliError> {
        let pool = tchatchers_core::pool::get_pg_pool().await;
        let messages = WsMessageContent::query_all_for_room(room_name, &pool).await;
        println!("Messages fetched from {room_name}");
        messages.iter().rev().for_each(|m| println!("{m:#?}\n"));
        Ok(())
    }

    pub async fn get_activity() -> Result<(), CliError> {
        let pool = tchatchers_core::pool::get_pg_pool().await;
        let result = WsMessageStats::get_activity(&pool).await;
        println!("Activity report\n---\n");
        result
            .iter()
            .enumerate()
            .for_each(|(i, m)| println!("- #{} : {m:#?}\n", i + 1));
        Ok(())
    }
}
