use std::{
    env::var,
    fs::{self, OpenOptions},
    io::Write,
    process::{Command, Output},
};

use dialoguer::{Confirm, Input, Password};
use which::which;

use crate::errors::CliError;

/// This struct provides functionality to interact with environment variables.
pub struct EnvAction;

const FILE_NAME: &str = ".env";

const CHECKMARK_EMOJI: &str = "\u{2714}";
const ERROR_EMOJI: &str = "\u{0058}";
const WARNING_EMOJI: &str = "\u{26A0}";

/// A constant array of tuples representing the environment variables that should be checked, along with their error types.
const ENV_VARS_TO_CHECK: [(&str, EnvironmentCheckErrorTypes); 7] = [
    ("DATABASE_URL", EnvironmentCheckErrorTypes::Warning),
    ("POSTGRES_DB", EnvironmentCheckErrorTypes::Error),
    ("POSTGRES_USER", EnvironmentCheckErrorTypes::Error),
    ("POSTGRES_PASSWORD", EnvironmentCheckErrorTypes::Error),
    ("JWT_SECRET", EnvironmentCheckErrorTypes::Error),
    ("UID", EnvironmentCheckErrorTypes::Warning),
    ("GID", EnvironmentCheckErrorTypes::Warning),
];

/// A constant array of program names to check if they exist in the PATH.
const PATH_PGM_TO_CHECK: [&str; 6] = [
    "docker",
    "cargo",
    "docker-compose",
    "npx",
    "trunk",
    "rustup",
];

/// A constant array of targets to check if they are installed for cargo.
const TARGETS_TO_CHECK: [&str; 1] = ["wasm32-unknown-unknown"];

/// An enum representing the types of errors for environment variable checks.
#[derive(Debug, PartialEq, Eq)]
enum EnvironmentCheckErrorTypes {
    Error,
    Warning,
}

impl EnvAction {
    /// Create a new `.env` file and populate it with database-related environment variables.

    pub fn create() -> Result<(), CliError> {
        if fs::read(FILE_NAME).is_ok() {
            let confirm_override = Confirm::new()
                .with_prompt("The .env file already exists, confirm that you want to override it.")
                .default(false)
                .interact()?;
            if !confirm_override {
                return Ok(());
            } else {
                println!("Completing this process will override the existing file ...");
            }
        } else {
            println!("Setting up a new .env file");
        }

        let postgres_host: String = Input::new()
            .with_prompt("Database host")
            .default("localhost".into())
            .interact_text()?;
        let postgres_db_name: String = Input::new()
            .with_prompt("Enter the database name")
            .default("chatapp".into())
            .interact_text()?;
        let postgres_user_name: String = Input::new()
            .with_prompt("Enter the database user name")
            .default("chatter".into())
            .interact_text()?;
        let postgres_password: String = Password::new()
            .with_prompt("Enter the DB password")
            .interact()?;
        let jwt_secret: String = Password::new()
            .with_prompt("Enter the JWT password")
            .interact()?;

        let mut env_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(FILE_NAME)?;
        writeln!(env_file, "DATABASE_URL=postgres://{postgres_user_name}:{postgres_password}@{postgres_host}/chatapp")?;
        writeln!(env_file, "POSTGRES_HOST={postgres_host}")?;
        writeln!(env_file, "POSTGRES_DB={postgres_db_name}")?;
        writeln!(env_file, "POSTGRES_USER={postgres_user_name}")?;
        writeln!(env_file, "POSTGRES_PASSWORD={postgres_password}")?;
        writeln!(env_file, "JWT_SECRET={jwt_secret}")?;

        Ok(())
    }

    /// Check the current .env file.
    pub async fn check_setup() -> Result<(), CliError> {
        let mut errors: Vec<EnvironmentCheckErrorTypes> = Vec::new();

        println!("\t---\n\tEnvironment variables\n\t---");
        for (env_var, err) in ENV_VARS_TO_CHECK {
            let emoji = match var(env_var).is_ok() {
                true => CHECKMARK_EMOJI,
                false => {
                    let emoji = match err {
                        EnvironmentCheckErrorTypes::Warning => WARNING_EMOJI,
                        EnvironmentCheckErrorTypes::Error => ERROR_EMOJI,
                    };
                    errors.push(err);
                    emoji
                }
            };
            println!("- [{emoji}] {env_var} set in ENV");
        }

        println!("\t---\n\tPrograms in path\n\t---");
        for pgm in PATH_PGM_TO_CHECK {
            let emoji: &str = match which(pgm).is_ok() {
                true => CHECKMARK_EMOJI,
                false => {
                    errors.push(EnvironmentCheckErrorTypes::Warning);
                    WARNING_EMOJI
                }
            };
            println!("- [{emoji}] {pgm}");
        }

        println!("\t---\n\tChecking cargo targets\n\t---");
        let targets: Output = Command::new("rustup")
            .arg("target")
            .arg("list")
            .arg("--installed")
            .output()?;

        match targets.status.success() {
            true => match std::str::from_utf8(&targets.stdout) {
                Ok(v) => {
                    for target in TARGETS_TO_CHECK {
                        let emoji = match v.contains(target) {
                            true => CHECKMARK_EMOJI,
                            false => {
                                errors.push(EnvironmentCheckErrorTypes::Warning);
                                WARNING_EMOJI
                            }
                        };
                        println!("- [{emoji}] {target}");
                    }
                }
                Err(_) => {
                    errors.push(EnvironmentCheckErrorTypes::Warning);
                    println!("- [{WARNING_EMOJI}] Couldn't get the available rustup targets.");
                }
            },
            false => {
                errors.push(EnvironmentCheckErrorTypes::Warning);
                println!("- [{WARNING_EMOJI}] Couldn't get the available rustup targets.");
            }
        }

        println!("\t---\n\tConnection to database\n\t---");
        tchatchers_core::pool::get_pg_pool().await;
        println!("- [{CHECKMARK_EMOJI}] Connection to database");

        println!("\t---\n");

        if errors.is_empty() {
            println!(
                "- [{CHECKMARK_EMOJI}] This set up is ready for either development or production."
            );
        } else {
            let errors_count = errors.len();
            let fatal_errors = errors
                .into_iter()
                .filter(|e| e == &EnvironmentCheckErrorTypes::Error)
                .count();
            if fatal_errors != 0usize {
                println!("- [{ERROR_EMOJI}] Some fatal errors were detected during the setup, please review it. This application will most likely not start.")
            } else {
                println!("- [{WARNING_EMOJI}] Some errors were detected but none of them were fatal, your app might start but you should ensure first that your .env is set up accordingly to your needs.")
            }
            println!("{fatal_errors} Fatal errors detected");
            println!("{} Warnings detected", errors_count - fatal_errors);
        }

        Ok(())
    }
}
