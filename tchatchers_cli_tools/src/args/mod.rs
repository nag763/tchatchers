use self::{env::EnvArgAction, user::UserArgAction};

pub mod env;
pub mod user;

#[derive(clap::Parser, Debug)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub entity: CliEntityArg,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum CliEntityArg {
    User {
        #[command(subcommand)]
        action: UserArgAction,
    },
    Room,
    Message,
    Env {
        action: EnvArgAction,
    },
}
