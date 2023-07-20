use crate::{
    common::user::{UserIdentifier, UserSearch},
    errors::CliError,
};
use dialoguer::{Confirm, Input, Password, Select};
use tchatchers_core::{
    profile::Profile,
    user::{InsertableUser, PartialUser, User}, locale::Locale,
};
use validator::Validate;

/// Struct containing functions to perform actions on users.
pub struct UserAction;

impl UserAction {
    /// Deletes a user from the database.
    ///
    /// # Arguments
    ///
    /// * `user_identifier` - The identifier of the user to delete.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the user was successfully deleted, or an error of type `CliError` if an error occurred during the operation.
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

    /// Updates the activation status of a user in the database.
    ///
    /// # Arguments
    ///
    /// * `user_identifier` - The identifier of the user to update.
    /// * `is_authorized` - The new activation status of the user.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the user was successfully updated, or an error of type `CliError` if an error occurred during the operation.
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

    /// Search for users based on the provided `UserSearch` criteria and print the result to the console.
    ///
    /// # Arguments
    ///
    /// * `user_search` - A `UserSearch` object that specifies the search criteria.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the operation is successful.
    /// * `Err(CliError)` if there is an error during the operation.
    pub async fn search_user(user_search: UserSearch) -> Result<(), CliError> {
        // Get the database connection pool.
        let pool = tchatchers_core::pool::get_pg_pool().await;

        // Perform the search based on the specified criteria.
        let result = match user_search {
            UserSearch::Id { value } => PartialUser::find_by_id(value, &pool)
                .await?
                .into_iter()
                .filter_map(Some)
                .collect::<Vec<PartialUser>>(),
            UserSearch::Login { value } => PartialUser::find_by_login(&value, &pool)
                .await?
                .into_iter()
                .filter_map(Some)
                .collect::<Vec<PartialUser>>(),
            UserSearch::Name { value } => PartialUser::find_by_name(&value, &pool).await?,
        };

        // Print the result to the console.
        if result.is_empty() {
            println!("No result found for your search criteria.")
        } else {
            for (i, user) in result.iter().enumerate() {
                println!("- {i} : {user:?}");
            }
        }

        Ok(())
    }

    /// Create a new user and insert it into the database.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the operation is successful.
    /// * `Err(CliError)` if there is an error during the operation.
    pub async fn create_user() -> Result<(), CliError> {
        // Get the database connection pool.
        let pool = tchatchers_core::pool::get_pg_pool().await;

        // Prompt the user to enter the user login.
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

        // Prompt the user to enter the user name.
        let user_name: String = Input::new().with_prompt("User name").interact_text()?;

        // Prompt the user to enter the user password.
        let password: String = Password::new()
            .with_prompt("Write the user password")
            .with_confirmation(
                "Confirm your password",
                "The passwords do not match, please retry",
            )
            .interact()?;

        // Prompt the user to select the user profile.
        let profiles = Profile::options();
        let profile_index = Select::new()
            .with_prompt("Select the user profile.")
            .items(&profiles)
            .default(Profile::User as usize)
            .interact()?;
        let profile = profiles[profile_index];

        // Create an `InsertableUser` object with the entered information.
        let insertable_user: InsertableUser = InsertableUser {
            login: user_login,
            password,
            name: user_name,
            locale: Locale::get_default_locale().id
        };

        // Validate the `InsertableUser` object, and prompt the user to confirm if there are validation errors.
        if insertable_user.validate().is_err() && !Confirm::new().with_prompt("The user you entered contains some validation errors.\nYou can valid that you want to persist it, but it is not recommended.").default(false).interact()? {
        } else {
            insertable_user.insert_with_profile(profile, &pool).await?;
            println!("The user has been created with success");
        }
        Ok(())
    }
}
