use crate::{common::user::{UserIdentifier, UserSearch}, errors::CliError};
use tchatchers_core::user::{User, PartialUser};

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
            UserSearch::Id { value } => PartialUser::find_by_id(value, &pool).await?.into_iter().filter_map(|v| Some(v)).collect::<Vec<PartialUser>>(),
            UserSearch::Login { value } => PartialUser::find_by_login(&value, &pool).await?.into_iter().filter_map(|v| Some(v)).collect::<Vec<PartialUser>>(),
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
}
