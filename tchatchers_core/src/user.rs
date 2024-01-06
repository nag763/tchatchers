// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! A user modelizes an authenticated client of the application.
//!
//! The user is declined under different structs so that only the revelant data
//! is shared between processed and components.

#[cfg(any(feature = "back", feature = "cli", feature = "async"))]
use crate::async_message::{AsyncOperationPGType, AsyncQueue};
use crate::authorization_status::AuthorizationStatus;
use crate::common::RE_LIMITED_CHARS;
use crate::profile::Profile;
use chrono::DateTime;
use chrono::Utc;
use derive_more::Display;
#[cfg(any(feature = "back", feature = "cli"))]
use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};
#[cfg(any(feature = "back", feature = "cli", feature = "async"))]
use sqlx::postgres::PgQueryResult;
#[cfg(any(feature = "back", feature = "cli", feature = "async"))]
use sqlx::FromRow;
#[cfg(any(feature = "back", feature = "cli", feature = "async"))]
use sqlx::PgPool;
use validator::Validate;
use validator::ValidationError;

lazy_static! {
    static ref RE_ONE_LOWERCASE_CHAR: Regex = Regex::new(r"[a-z]+").unwrap();
    static ref RE_ONE_NUMBER: Regex = Regex::new(r"[0-9]+").unwrap();
    static ref RE_ONE_UPPERCASE_CHAR: Regex = Regex::new(r"[A-Z]+").unwrap();
}

/// The in base structure, which should never be shared between components and
/// apps.
#[derive(Serialize, Deserialize, Debug, derivative::Derivative)]
#[cfg_attr(
    any(feature = "back", feature = "cli", feature = "async"),
    derive(FromRow)
)]
#[serde(rename_all = "camelCase")]
#[derivative(Default)]
pub struct User {
    /// The in base id, unique.
    pub id: i32,
    /// The login of the user, should also be unique.
    pub login: String,
    /// The user password, should always be encrypted and secured.
    pub password: String,
    /// Whether the user is authorized to connect to the different services.
    ///
    /// If set to false, the user shouldn't be allowed to connect to the apps.
    #[cfg_attr(
        any(feature = "back", feature = "cli", feature = "async"),
        sqlx(rename = "authorization_status_id")
    )]
    pub authorization_status: AuthorizationStatus,
    /// The name of the user, should be displayed on front end services.
    pub name: String,
    /// The profile picture of the user.
    pub pfp: Option<String>,
    /// The locale associated with the user.
    pub locale_id: i32,
    /// Timestamp when the user got created.
    #[derivative(Default(value = "chrono::offset::Utc::now()"))]
    pub created_at: DateTime<Utc>,
    /// Timestamp on when the user got updated the last time.
    #[derivative(Default(value = "chrono::offset::Utc::now()"))]
    pub last_update: DateTime<Utc>,
    /// Timestamp on when the user logged on the last time.
    pub last_logon: Option<DateTime<Utc>>,
    /// The user's profile.
    #[cfg_attr(
        any(feature = "back", feature = "cli", feature = "async"),
        sqlx(rename = "profile_id")
    )]
    pub profile: Profile,
    /// The user's email.
    pub email: Option<String>,
}

#[cfg(any(feature = "back", feature = "cli", feature = "async"))]
impl User {
    pub fn is_authorized(&self) -> bool {
        if self.email.is_some() {
            self.authorization_status.is_authorized()
        } else {
            !self.authorization_status.is_deactivated()
        }
    }

    /// Find a user by ID in the database.
    ///
    /// This shouldn't be used to identify users.
    ///
    /// # Arguments
    ///
    /// - id : The id of the user we are looking for.
    /// - pool : The pool of connection.
    pub async fn find_by_id(id: i32, pool: &PgPool) -> Result<Option<Self>, sqlx::error::Error> {
        sqlx::query_as("SELECT * FROM CHATTER WHERE id=$1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    /// Look up whether a login exists in the database.
    ///
    /// # Arguments
    ///
    /// - login : The login to look up.
    pub async fn login_exists(login: &str, pool: &PgPool) -> Result<bool, sqlx::Error> {
        let row: (bool,) = sqlx::query_as("SELECT COUNT(id)!=0 FROM CHATTER WHERE login=$1")
            .bind(login)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }

    /// Look up whether a login exists in the database.
    ///
    /// # Arguments
    ///
    /// - login : The login to look up.
    pub async fn email_exists(email: &str, pool: &PgPool) -> Result<bool, sqlx::Error> {
        let row: (bool,) = sqlx::query_as("SELECT COUNT(id)!=0 FROM CHATTER WHERE email ilike $1")
            .bind(email)
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }

    /// Look up whether a login exists in the database.
    ///
    /// # Arguments
    ///
    /// - login : The login to look up.
    pub async fn email_exists_except_self(
        email: &str,
        id: i32,
        pool: &PgPool,
    ) -> Result<bool, sqlx::Error> {
        let row: (bool,) =
            sqlx::query_as("SELECT COUNT(id)!=0 FROM CHATTER WHERE email ilike $1 AND id != $2")
                .bind(email)
                .bind(id)
                .fetch_one(pool)
                .await?;
        Ok(row.0)
    }

    /// Delete the user from the database.
    ///
    /// The check on whether the executer can delete the user has to be done server side.
    /// This won't check the legitimity of the operation.
    ///
    /// # Arguments
    ///
    /// - id : the user to delete.
    /// - pool : The PG connection pool.
    pub async fn delete_one(id: i32, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("DELETE FROM CHATTER WHERE id=$1")
            .bind(id)
            .execute(pool)
            .await
    }

    /// Delete the user from the database.
    ///
    /// The check on whether the executer can delete the user has to be done server side.
    /// This won't check the legitimity of the operation.
    ///
    /// # Arguments
    ///
    /// - id : the user to delete.
    /// - pool : The PG connection pool.
    pub async fn delete_login(login: &str, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("DELETE FROM CHATTER WHERE login=$1")
            .bind(login)
            .execute(pool)
            .await
    }

    /// Delete the user from the database.
    ///
    /// The check on whether the executer can delete the user has to be done server side.
    /// This won't check the legitimity of the operation.
    ///
    /// # Arguments
    ///
    /// - email : the user's email to delete.
    /// - pool : The PG connection pool.
    pub async fn delete_mail(email: &str, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("DELETE FROM CHATTER WHERE email ilike $1")
            .bind(email)
            .execute(pool)
            .await
    }

    /// Update the activation status of a user.
    ///
    /// This will mark a user as either authorized or unauthorized on the base.
    ///
    /// # Arguments
    ///
    /// - id : the user id.
    /// - is_authorized : the new activation status.
    pub async fn update_activation_status(
        id: i32,
        authorization_status: AuthorizationStatus,
        pool: &PgPool,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("UPDATE CHATTER SET authorization_status_id=$1 WHERE id=$2")
            .bind(authorization_status)
            .bind(id)
            .execute(pool)
            .await
    }

    /// Update the activation status of a user.
    ///
    /// This will mark a user as either authorized or unauthorized on the base.
    ///
    /// # Arguments
    ///
    /// - login : the user login.
    /// - is_authorized : the new activation status.
    pub async fn update_activation_status_from_login(
        login: &str,
        authorization_status: AuthorizationStatus,
        pool: &PgPool,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("UPDATE CHATTER SET authorization_status=$1 WHERE login=$2")
            .bind(authorization_status)
            .bind(login)
            .execute(pool)
            .await
    }

    pub async fn update_activation_status_from_mail(
        email: &str,
        is_authorized: bool,
        pool: &PgPool,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("UPDATE CHATTER SET is_authorized=$1 WHERE email ilike $2")
            .bind(is_authorized)
            .bind(email)
            .execute(pool)
            .await
    }

    /// Marks users as logged based on the provided user identifier and updates the `LAST_LOGON` field in the database.
    ///
    /// This function creates a temporary table to store the user updates and performs the necessary SQL operations to mark users as logged.
    ///
    /// # Arguments
    ///
    /// * `userid_identifier` - A vector of `AsyncOperationPGType<i32>` representing the user IDs and associated queue IDs and timestamps.
    /// * `pool` - A reference to the PostgreSQL pool for database operations.
    ///
    /// # Returns
    ///
    /// An empty result indicating success, or an `sqlx::Error` if an error occurs during database operations.
    ///
    pub async fn mark_users_as_logged(
        userid_identifier: Vec<AsyncOperationPGType<i32>>,
        pool: &PgPool,
    ) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        sqlx::query(
            "
            CREATE TEMPORARY TABLE tmp_user_update(
                entity_id integer NOT NULL UNIQUE,
                queue_id text NOT NULL,
                timestamp TIMESTAMPTZ NOT NULL,
                is_updated boolean default false
            ) ON COMMIT DROP;
        ",
        )
        .execute(&mut *tx)
        .await?;

        for operation in userid_identifier {
            sqlx::query(
                "
                INSERT INTO tmp_user_update(entity_id, queue_id, timestamp) 
                VALUES ($1, $2, $3)
                ON CONFLICT ON CONSTRAINT tmp_user_update_entity_id_key
                DO NOTHING",
            )
            .bind(operation.entity_id)
            .bind(operation.queue_id)
            .bind(operation.timestamp)
            .execute(&mut *tx)
            .await?;
        }

        sqlx::query("UPDATE CHATTER c SET LAST_LOGON = tr.timestamp FROM tmp_user_update tr WHERE tr.entity_id = c.id").execute(&mut *tx).await?;
        sqlx::query("UPDATE tmp_user_update tr SET is_updated=true FROM CHATTER c WHERE tr.entity_id = c.id").execute(&mut *tx).await?;

        sqlx::query("
        INSERT INTO PROCESS_REPORT(process_id, successfull_records, failed_records) 
        SELECT $1, sum(case when is_updated then 1 else 0 end), sum(case when is_updated then 0 else 1 end) 
        FROM tmp_user_update
        ")
            .bind(AsyncQueue::LoggedUsers as i32)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }
}

/// Structure mostly used to share the data between the applications.
///
/// It is containing limited data, which is convenient and secure during
/// exchanges. Thus, this is the struct used in JWT.
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    derivative::Derivative,
    PartialEq,
    Eq,
    Hash,
    Display,
)]
#[cfg_attr(
    any(feature = "back", feature = "cli", feature = "async"),
    derive(FromRow)
)]
#[display(fmt = "[{id}::{login}] {name} ({profile})")]
#[serde(rename_all = "camelCase")]
#[derivative(Default)]
pub struct PartialUser {
    /// In base id of the user.
    pub id: i32,
    /// Login of the user.
    pub login: String,
    /// Name of the user.
    pub name: String,
    /// Profile picture of the user.
    pub pfp: Option<String>,
    /// Whether the user is authorized or not.
    #[cfg_attr(
        any(feature = "back", feature = "cli", feature = "async"),
        sqlx(rename = "authorization_status_id")
    )]
    pub authorization_status: AuthorizationStatus,
    /// Timestamp when the user got created.
    #[derivative(Default(value = "chrono::offset::Utc::now()"))]
    pub created_at: DateTime<Utc>,
    /// Timestamp on when the user got updated the last time.
    #[derivative(Default(value = "chrono::offset::Utc::now()"))]
    pub last_update: DateTime<Utc>,
    /// Timestamp on when the user logged on the last time.
    pub last_logon: Option<DateTime<Utc>>,
    /// The locale associated with the user.
    pub locale_id: i32,
    // The profile of the user.
    #[cfg_attr(any(feature = "back", feature = "cli"), sqlx(rename = "profile_id"))]
    pub profile: Profile,
    // User's email
    pub email: Option<String>,
}

impl From<User> for PartialUser {
    fn from(user: User) -> PartialUser {
        PartialUser {
            id: user.id,
            login: user.login,
            name: user.name,
            pfp: user.pfp,
            locale_id: user.locale_id,
            profile: user.profile,
            authorization_status: user.authorization_status,
            created_at: user.created_at,
            last_update: user.last_update,
            last_logon: user.last_logon,
            email: user.email,
        }
    }
}

#[cfg(any(feature = "back", feature = "cli", feature = "async"))]
impl PartialUser {
    /// Find a user by ID in the database.
    ///
    /// This shouldn't be used to identify users.
    ///
    /// # Arguments
    ///
    /// - id : The id of the user we are looking for.
    /// - pool : The pool of connection.
    pub async fn find_by_id(id: i32, pool: &PgPool) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM CHATTER WHERE id=$1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    /// Find a user from his login.
    ///
    /// This is an exact match look up.
    ///
    /// # Arguments
    ///
    /// - login : the user login.
    pub async fn find_by_login(login: &str, pool: &PgPool) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM CHATTER WHERE login=$1")
            .bind(login)
            .fetch_optional(pool)
            .await
    }

    /// Find a user from his email.
    ///
    /// This is an exact match look up.
    ///
    /// # Arguments
    ///
    /// - email : the user email.
    pub async fn find_by_email(email: &str, pool: &PgPool) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM CHATTER WHERE email ilike $1")
            .bind(email)
            .fetch_optional(pool)
            .await
    }

    /// Find a user from his name.
    ///
    /// This is an exact match look up.
    ///
    /// # Arguments
    ///
    /// - name : the user name.
    pub async fn find_by_name(name: &str, pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM CHATTER WHERE name=$1")
            .bind(name)
            .fetch_all(pool)
            .await
    }

    pub(crate) async fn clear_data(
        entities_to_clear: std::collections::HashSet<&PartialUser>,
        pool: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        let mut file_removers: tokio::task::JoinSet<tokio::io::Result<()>> =
            tokio::task::JoinSet::new();

        for entity in entities_to_clear {
            if let Some(pfp) = &entity.pfp {
                let path = std::path::Path::new(&pfp);
                if let Some(filename) = path.file_name() {
                    let file_path = format!("./static/{}", filename.to_str().unwrap());
                    debug!(
                        "[{}] File path to delete is {file_path}",
                        AsyncQueue::RemoveUserData
                    );
                    file_removers.spawn(tokio::fs::remove_file(file_path));
                }
            }
        }

        let total = file_removers.len();
        let mut err = 0usize;

        while let Some(handle) = file_removers.join_next().await {
            let res = handle;
            if let Ok(res) = res {
                if let Err(e) = res {
                    error!(
                        "[{}] Error while attempting to delete the file : {e}",
                        AsyncQueue::RemoveUserData
                    );
                    err += 1;
                }
            } else {
                error!(
                    "[{}] Error met while accessing the async process's results",
                    AsyncQueue::RemoveUserData
                );
                err += 1;
            }
        }

        sqlx::query(
            "
        INSERT INTO PROCESS_REPORT(process_id, successfull_records, failed_records) VALUES ($1, $2, $3)
        ",
        )
        .bind(AsyncQueue::RemoveUserData as i32)
        .bind((total - err) as i64)
        .bind(err as i64)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }
}

fn password_strengh(password: &str) -> Result<(), ValidationError> {
    if !RE_ONE_LOWERCASE_CHAR.is_match(password)
        || !RE_ONE_UPPERCASE_CHAR.is_match(password)
        || !RE_ONE_NUMBER.is_match(password)
    {
        Err(ValidationError::new("security_constraints_not_matched"))
    } else {
        Ok(())
    }
}

/// Structure used only to create new DB entities.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsertableUser {
    /// The user log in.
    #[validate(
        length(min = 3, max = 32),
        regex(path = "RE_LIMITED_CHARS", code = "limited_chars")
    )]
    pub login: String,
    /// The user password, should be raw prior being insert.
    #[validate(
        length(min = 8, max = 128),
        custom(
            function = "password_strengh",
            code = "security_constraints_not_matched"
        )
    )]
    pub password: String,
    /// The name of the user.
    #[validate(
        length(min = 3, max = 16),
        regex(path = "RE_LIMITED_CHARS", code = "limited_chars")
    )]
    pub name: String,
    /// The user's email.
    #[validate(email)]
    pub email: Option<String>,
    /// The user's locale
    #[validate(range(min = 1))]
    pub locale: i32,
}

impl InsertableUser {
    /// Inserts the user in the database.
    ///
    /// # Arguments
    ///
    /// - pool : The connection pool.
    #[cfg(feature = "back")]
    pub async fn insert(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = argon2::Config::rfc9106_low_mem();
        let hash = argon2::hash_encoded(self.password.as_bytes(), &salt, &config).unwrap();
        sqlx::query(
            "INSERT INTO CHATTER(login, password, name, locale_id, email) VALUES ($1,$2,$3,$4,$5)",
        )
        .bind(&self.login)
        .bind(&hash)
        .bind(&self.name)
        .bind(self.locale)
        .bind(&self.email)
        .execute(pool)
        .await
    }

    /// Inserts the user in the database along his profile.
    ///
    /// # Arguments
    ///
    /// - profile : The user profile.
    /// - pool : The connection pool.
    #[cfg(feature = "cli")]
    pub async fn insert_with_profile(
        &self,
        profile: Profile,
        pool: &PgPool,
    ) -> Result<PgQueryResult, sqlx::Error> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = argon2::Config::rfc9106_low_mem();
        let hash = argon2::hash_encoded(self.password.as_bytes(), &salt, &config).unwrap();
        sqlx::query(
            "INSERT INTO CHATTER(login, password, name, profile_id, email) VALUES ($1,$2,$3,$4,$5)",
        )
        .bind(&self.login)
        .bind(&hash)
        .bind(&self.name)
        .bind(profile)
        .bind(&self.email)
        .execute(pool)
        .await
    }
}

/// The updatabale structure, should only be used to update a db entity.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdatableUser {
    pub id: i32,
    #[validate(
        length(min = 3, max = 16),
        regex(path = "RE_LIMITED_CHARS", code = "limited_chars")
    )]
    pub name: String,
    pub locale_id: i32,
    #[validate(email)]
    pub email: Option<String>,
}

#[cfg(feature = "back")]
impl UpdatableUser {
    /// Updates the user in the database.
    ///
    /// # Arguments
    ///
    /// - pool : The connection pool.
    pub async fn update(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("UPDATE CHATTER SET name=$1, locale_id=$2, email=$3 WHERE id=$4")
            .bind(&self.name)
            .bind(self.locale_id)
            .bind(&self.email)
            .bind(self.id)
            .execute(pool)
            .await
    }

    pub async fn set_pfp(id: i32, pfp: &str, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("UPDATE CHATTER SET pfp=$1 WHERE id=$2")
            .bind(pfp)
            .bind(id)
            .execute(pool)
            .await
    }
}

/// Structure used to authenticate a user.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Validate)]
pub struct AuthenticableUser {
    /// The login of the user, has to exist in database.
    pub login: String,
    /// The password of the user, has to be encrypted.
    pub password: String,
    /// If the session of the user has to be persisted.
    pub session_only: bool,
}

#[cfg(feature = "back")]
impl AuthenticableUser {
    /// Authenticate the user in the database.
    ///
    /// It will return the full structure if the authentication is successful.
    ///
    /// # Arguments
    ///
    /// - pool : The connection pool.
    pub async fn authenticate(&self, pool: &PgPool) -> Result<Option<User>, sqlx::error::Error> {
        let user: Option<User> = sqlx::query_as("SELECT * FROM CHATTER WHERE login=$1")
            .bind(&self.login)
            .fetch_optional(pool)
            .await?;
        let Some(user) = user else {
            return Ok(None);
        };
        match argon2::verify_encoded(&user.password, self.password.as_bytes()).unwrap() {
            true => Ok(Some(user)),
            false => Ok(None),
        }
    }
}
