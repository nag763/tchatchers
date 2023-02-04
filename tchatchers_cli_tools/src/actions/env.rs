use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use dialoguer::{Confirm, Input, Password};

use crate::errors::CliError;

pub struct EnvAction;

const FILE_NAME: &str = ".env";

impl EnvAction {
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
        writeln!(env_file, "POSTGRES_DB={postgres_db_name}")?;
        writeln!(env_file, "POSTGRES_USER={postgres_user_name}")?;
        writeln!(env_file, "POSTGRES_PASSWORD={postgres_password}")?;
        writeln!(env_file, "JWT_SECRET={jwt_secret}")?;

        Ok(())
    }
}
