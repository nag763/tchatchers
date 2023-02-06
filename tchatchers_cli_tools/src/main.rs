mod actions;
mod args;
mod common;
mod errors;

use std::process::{ExitCode, Termination};

use actions::env::EnvAction;
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
            args::user::UserArgAction::Create => todo!(),
            args::user::UserArgAction::Update { id: _id } => todo!(),
            args::user::UserArgAction::Deactivate { user_identifier } => {
                UserAction::update_activation_status(user_identifier, false).await?
            }
            args::user::UserArgAction::Activate { user_identifier } => {
                UserAction::update_activation_status(user_identifier, true).await?
            }
            args::user::UserArgAction::Delete { user_identifier } => {
                UserAction::delete_user(user_identifier).await?
            }
            args::user::UserArgAction::Search { user_search } => UserAction::search_user(user_search).await?,
        },
        args::CliEntityArg::Room => todo!(),
        args::CliEntityArg::Message => todo!(),
        args::CliEntityArg::Env { action } => match action {
            args::env::EnvArgAction::Create => EnvAction::create().unwrap(),
            args::env::EnvArgAction::Check => todo!(),
        },
    }
    Ok(())
}
