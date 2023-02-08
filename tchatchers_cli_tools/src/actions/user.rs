use crate::{
    common::user::{UserIdentifier, UserSearch},
    errors::CliError,
};
use dialoguer::{Confirm, Input, Password, Select};
use tchatchers_core::{
    profile::Profile,
    user::{InsertableUser, PartialUser, User},
};
use validator::Validate;

pub struct UserAction;

impl UserAction {
    pub async fn delete_user(user_identifier: UserIdentifier) -> Result<(), CliError> {
        let pool = tchatchers_core::pool::get_pg_pool().await;
        let result = match user_identifier {
            UserIdentifier::Id { value } => User::delete_one(value, &pool).await?,
            UserIdentifier::Login { value } => User::delete_login(&value, &pool).await?,
        };
        if result.rows_affected() == 1 {
            println!("The user has been deleted with success.");
        } else {
            eprintln!("The user wasn't found during the operation. Nothing has been updated in consequence.");
        }
        Ok(())
    }

    pub async fn update_activation_status(
        user_identifier: UserIdentifier,
        is_authorized: bool,
    ) -> Result<(), CliError> {
        let pool = tchatchers_core::pool::get_pg_pool().await;
        let result = match user_identifier {
            UserIdentifier::Id { value } => {
                User::update_activation_status(value, is_authorized, &pool).await?
            }
            UserIdentifier::Login { value } => {
                User::update_activation_status_from_login(&value, is_authorized, &pool).await?
            }
        };
        if result.rows_affected() == 1 {
            println!("The user has been updated with success.");
        } else {
            eprintln!("The user wasn't found during the operation. Nothing has been updated in consequence.");
        }
        Ok(())
    }

    pub async fn search_user(user_search: UserSearch) -> Result<(), CliError> {
        let pool = tchatchers_core::pool::get_pg_pool().await;
        let result = match user_search {
            UserSearch::Id { value } => PartialUser::find_by_id(value, &pool)
                .await?
                .into_iter()
                .filter_map(|v| Some(v))
                .collect::<Vec<PartialUser>>(),
            UserSearch::Login { value } => PartialUser::find_by_login(&value, &pool)
                .await?
                .into_iter()
                .filter_map(|v| Some(v))
                .collect::<Vec<PartialUser>>(),
            UserSearch::Name { value } => PartialUser::find_by_name(&value, &pool).await?,
        };
        if result.is_empty() {
            println!("No result found for your search criteria.")
        } else {
            for (i, user) in result.iter().enumerate() {
                println!("- {i} : {user:?}");
            }
        }
        Ok(())
    }

    pub async fn create_user() -> Result<(), CliError> {
        let pool = tchatchers_core::pool::get_pg_pool().await;
        let user_login: String = loop {
            let login: String = Input::new()
                .with_prompt("Write the user's login")
                .interact_text()?;
            if !User::login_exists(&login, &pool).await {
                break login;
            } else {
                println!(
                    "This login is already taken by another user.\nPlease try with another one."
                );
            }
        };
        let user_name: String = Input::new().with_prompt("User name").interact_text()?;
        let password: String = Password::new()
            .with_prompt("Write the user password")
            .with_confirmation(
                "Confirm your password",
                "The passwords do not match, please retry",
            )
            .interact()?;

        let profiles = Profile::options();
        let profile_index = Select::new()
            .with_prompt("Select the user profile.")
            .items(&profiles)
            .default(Profile::User as usize)
            .interact()?;

        let profile = profiles[profile_index];

        let insertable_user: InsertableUser = InsertableUser {
            login: user_login,
            password,
            name: user_name,
        };

        if insertable_user.validate().is_err() && !Confirm::new().with_prompt("The user you entered contains some validation errors.\nYou can valid that you want to persist it, but it is not recommended.").default(false).interact()? {
        } else {
            insertable_user.insert_with_profile(profile, &pool).await?;
            println!("The user has been created with success");
        }
        Ok(())
    }
}
