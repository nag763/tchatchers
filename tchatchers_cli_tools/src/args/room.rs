/// The actions that can be ran on the rooms.
#[derive(Debug, Clone, clap::Subcommand)]
pub enum RoomArgAction {
    #[command(about = "Deletes all the messages in a room")]
    Clean { room_name: String },
    #[command(about = "Returns all the messages in a room, from the latest to the oldest")]
    GetMessages { room_name: String },
    #[command(about = "Print global activity of the application's rooms")]
    Activity,
}
