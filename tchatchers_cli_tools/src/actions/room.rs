use crate::errors::CliError;
use tchatchers_core::ws_message::{WsMessageContent, WsMessageStats};

/// Struct for performing actions related to chat rooms.
pub struct RoomAction;

impl RoomAction {
    /// Asynchronously deletes all messages in the specified chat room.
    ///
    /// # Arguments
    ///
    /// * `room_name` - The name of the chat room to delete messages from.
    ///
    /// # Errors
    ///
    /// Returns a `CliError` if the operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn test_delete_messages() -> Result<(), CliError> {
    /// # use my_chat_app::RoomAction;
    /// # let room_name = "test_room";
    /// RoomAction::delete_messages(room_name).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_messages(room_name: &str) -> Result<(), CliError> {
        let pool = tchatchers_core::pool::get_pg_pool().await?;
        let result = WsMessageContent::delete_message_in_room(room_name, &pool).await?;
        println!(
            "{} messages deleted in room '{}'",
            result.rows_affected(),
            room_name
        );
        Ok(())
    }

    /// Asynchronously retrieves all messages in the specified chat room and prints them to the console.
    ///
    /// # Arguments
    ///
    /// * `room_name` - The name of the chat room to retrieve messages from.
    ///
    /// # Errors
    ///
    /// Returns a `CliError` if the operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn test_get_messages() -> Result<(), CliError> {
    /// # use my_chat_app::RoomAction;
    /// # let room_name = "test_room";
    /// RoomAction::get_messages(room_name).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_messages(room_name: &str) -> Result<(), CliError> {
        let pool = tchatchers_core::pool::get_pg_pool().await?;
        let messages = WsMessageContent::query_all_for_room(room_name, &pool).await;
        println!("Messages fetched from {}", room_name);
        messages.iter().rev().for_each(|m| println!("{m:#?}\n"));
        Ok(())
    }

    /// Asynchronously retrieves activity statistics for all chat rooms and prints them to the console.
    ///
    /// # Errors
    ///
    /// Returns a `CliError` if the operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn test_get_activity() -> Result<(), CliError> {
    /// # use my_chat_app::RoomAction;
    /// RoomAction::get_activity().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_activity() -> Result<(), CliError> {
        let pool = tchatchers_core::pool::get_pg_pool().await?;
        let result = WsMessageStats::get_activity(&pool).await;
        println!("Activity report\n---\n");
        result
            .iter()
            .enumerate()
            .for_each(|(i, m)| println!("- #{} : {m:#?}\n", i + 1));
        Ok(())
    }
}
