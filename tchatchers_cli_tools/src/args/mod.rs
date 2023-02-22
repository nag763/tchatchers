use self::{
    env::EnvArgAction, message::MessageArgAction, room::RoomArgAction, user::UserArgAction,
};

pub mod env;
pub mod message;
pub mod room;
pub mod user;

#[derive(clap::Parser, Debug)]
#[command(author="LABEYE Loïc", version, about="tct is a CLI tool designed to run the common operations on the tchatchers_app", after_help="This tool is meant to be used within the tchatchers project.\nRun it only either within the workspace (dev) or as a binary (prod).")]
pub struct CliArgs {
    #[clap(subcommand)]
    pub entity: CliEntityArg,
    #[clap(long, short, value_hint = clap::ValueHint::FilePath, required=false, global=true, help="Precise a .env file.")]
    pub env: Option<std::path::PathBuf>,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum CliEntityArg {
    #[command(about="Manages the users of the application")]    
    User {
        #[command(subcommand)]
        action: UserArgAction,
    },
    #[command(about="Manages the rooms of the application")]
    Room {
        #[command(subcommand)]
        action: RoomArgAction,
    },
    #[command(about="Manages the messages stored in the database")]
    Message {
        #[command(subcommand)]
        action: MessageArgAction,
    },
    #[command(about="Helper to either set up a new environment or check the current one")]
    Env {
        action: EnvArgAction,
    },
}
