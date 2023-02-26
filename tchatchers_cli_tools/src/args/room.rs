/// The actions that can be performed on the rooms, which are entities that store messages sent by users.
#[derive(Debug, Clone, clap::Subcommand)]
pub enum RoomArgAction {
    /// Deletes all the messages in a room.
    #[command(about = "Deletes all the messages in a room")]
    Clean { room_name: String },
    /// Returns all the messages in a room, from the latest to the oldest.
    #[command(about = "Returns all the messages in a room, from the latest to the oldest")]
    GetMessages { room_name: String },
    /// Prints global activity of the application's rooms.
    #[command(about = "Print global activity of the application's rooms")]
    Activity,
}
