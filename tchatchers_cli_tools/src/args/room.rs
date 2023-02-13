#[derive(Debug, Clone, clap::Subcommand)]
pub enum RoomArgAction {
    Clean { room_name: String },
    GetMessages { room_name: String },
    Activity,
}
