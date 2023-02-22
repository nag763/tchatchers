use self::{
    env::EnvArgAction, message::MessageArgAction, room::RoomArgAction, user::UserArgAction,
};

pub mod env;
pub mod message;
pub mod room;
pub mod user;

#[derive(clap::Parser, Debug)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub entity: CliEntityArg,
    #[clap(long, value_hint = clap::ValueHint::FilePath, required=false, global=true)]
    pub env: Option<std::path::PathBuf>,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum CliEntityArg {
    User {
        #[command(subcommand)]
        action: UserArgAction,
    },
    Room {
        #[command(subcommand)]
        action: RoomArgAction,
    },
    Message {
        #[command(subcommand)]
        action: MessageArgAction,
    },
    Env {
        action: EnvArgAction,
    },
}
