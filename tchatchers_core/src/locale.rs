// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! The locale is corresponds to a reference that can be used to translate some labels to the user.
//! 
//! The default locale is considered to be the english one, but since every user can use his language, 
//! this entity provides the tools to translates the app while browsing.

use serde::{Deserialize, Serialize};


/// The locale is a reference to translate the application.
/// 
/// A locale inherits usually from a language, and is more specific
/// to match a subgroup of a language users (ie french speaking canadians).
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "back", derive(sqlx::FromRow))]
pub struct Locale {
    /// In base id.
    pub id: i32,
    /// The parent language id.
    pub language_id: i32,
    /// The locale shortname.
    pub short_name: String,
    /// The locale's longname.
    pub long_name: String,
}

#[cfg(feature = "back")]
impl Locale {
    /// Returns all the locales stored in database.
    /// 
    /// # Arguments
    /// - pool : The postgres connection pool.
    pub(crate) async fn get_all(pool: &sqlx::PgPool) -> Vec<Locale> {
        sqlx::query_as("SELECT * FROM LOCALE")
            .fetch_all(pool)
            .await
            .unwrap()
    }

    /// Returns all the locales sorted alphabeticly by their name.
    /// 
    /// # Arguments
    /// 
    /// - pool : The postgres pool.
    pub(crate) async fn get_all_sorted_by_name(pool: &sqlx::PgPool) -> Vec<Locale> {
        sqlx::query_as("SELECT * FROM LOCALE ORDER BY long_name")
            .fetch_all(pool)
            .await
            .unwrap()
    }
}

/// The locale manager is used to store in a reloadable cache 
/// the different locales.
/// 
/// This is mainly used for performance reasons (fetch once, reload when needed). 
#[cfg(feature = "back")]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LocaleManager {
    /// The list of locales stored by the manager.
    locales: Vec<Locale>,
    /// Whether the manager has been loaded or not.
    init: bool,
}

#[cfg(feature = "back")]
impl LocaleManager {

    /// Initialise the locale manager
    /// 
    /// # Arguments
    /// - pg_pool : The pool used to fetch the locales.
    pub async fn init(pg_pool: &sqlx::PgPool) -> LocaleManager {
        let locales = Locale::get_all_sorted_by_name(pg_pool).await;
        LocaleManager {
            locales,
            init: true,
        }
    }

    /// Returns a locale from its id from the cache.
    /// 
    /// # Arguments
    /// 
    /// - locale_id : The id to fetch
    pub fn get(&self, locale_id: i32) -> Result<Locale, crate::manager::ManagerError<i32>> {
        if !self.init {
            Err(crate::manager::ManagerError::NotInit)
        } else if let Some(value) = self.locales.iter().find(|l| l.id == locale_id) {
            Ok(value.clone())
        } else {
            Err(crate::manager::ManagerError::NotBound(locale_id))
        }
    }

    /// Returns all the locale from the cache.
    pub fn get_all(&self) -> Result<Vec<Locale>, crate::manager::ManagerError<i32>> {
        if !self.init {
            Err(crate::manager::ManagerError::NotInit)
        } else {
            Ok(self.locales.clone())
        }
    }
}
