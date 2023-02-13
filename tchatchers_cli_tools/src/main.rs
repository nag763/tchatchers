mod actions;
mod args;
mod common;
mod errors;

use std::process::{ExitCode, Termination};

use actions::{env::EnvAction, room::RoomAction};
use args::CliArgs;
use clap::Parser;
use errors::CliError;

use crate::actions::user::UserAction;

#[macro_use]
extern crate derive_more;

#[tokio::main]
async fn main() -> ExitCode {
    match run_main().await {
        Ok(()) => {
            println!("The process ended with success.");
            ExitCode::SUCCESS
        }
        Err(err) => err.report(),
    }
}

async fn run_main() -> Result<(), CliError> {
    dotenv::dotenv().ok();
    let args: CliArgs = CliArgs::parse();
    match args.entity {
        args::CliEntityArg::User { action } => match action {
            args::user::UserArgAction::Create => UserAction::create_user().await?,
            args::user::UserArgAction::Deactivate { user_identifier } => {
                UserAction::update_activation_status(user_identifier, false).await?
            }
            args::user::UserArgAction::Activate { user_identifier } => {
                UserAction::update_activation_status(user_identifier, true).await?
            }
            args::user::UserArgAction::Delete { user_identifier } => {
                UserAction::delete_user(user_identifier).await?
            }
            args::user::UserArgAction::Search { user_search } => {
                UserAction::search_user(user_search).await?
            }
        },
        args::CliEntityArg::Message => todo!(),
        args::CliEntityArg::Env { action } => match action {
            args::env::EnvArgAction::Create => EnvAction::create()?,
            args::env::EnvArgAction::Check => EnvAction::check_setup().await?,
        },
        args::CliEntityArg::Room { action } => match action {
            args::room::RoomArgAction::Clean { room_name } => {
                RoomAction::delete_messages(&room_name).await?
            }
            args::room::RoomArgAction::GetMessages { room_name } => {
                RoomAction::get_messages(&room_name).await?
            }
            args::room::RoomArgAction::Activity => RoomAction::get_activity().await?,
        },
    }
    Ok(())
}
