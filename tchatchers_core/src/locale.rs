use serde::{Deserialize, Serialize};

use crate::manager::ManagerError;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "back", derive(sqlx::FromRow))]
pub struct Locale {
    pub id: i32,
    pub language_id: i32,
    pub short_name: String,
    pub long_name: String,
}

#[cfg(feature = "back")]
impl Locale {
    pub async fn get_all(pool: &sqlx::PgPool) -> Vec<Locale> {
        sqlx::query_as("SELECT * FROM LOCALE")
            .fetch_all(pool)
            .await
            .unwrap()
    }

    pub async fn get_all_sorted_by_name(pool: &sqlx::PgPool) -> Vec<Locale> {
        sqlx::query_as("SELECT * FROM LOCALE ORDER BY long_name")
            .fetch_all(pool)
            .await
            .unwrap()
    }

    pub async fn get_locale_id(locale_id: i32, pool: &sqlx::PgPool) -> Option<Locale> {
        sqlx::query_as("SELECT * FROM LOCALE loc WHERE id = $1")
            .bind(locale_id)
            .fetch_optional(pool)
            .await
            .unwrap()
    }
}

#[cfg(feature="back")]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LocaleManager {
    locales: std::collections::HashMap<i32, Locale>,
    init: bool
}

#[cfg(feature="back")]
impl LocaleManager {

    pub async fn init(pg_pool: &sqlx::PgPool) -> LocaleManager {
        let locales : std::collections::HashMap<i32, Locale> = Locale::get_all_sorted_by_name(pg_pool).await.iter().map(|l| (l.id, l.clone())).collect();
        LocaleManager {
            locales,
            init: true
        }
    }

    pub fn get(&self, locale_id : i32) -> Result<Locale, crate::manager::ManagerError<i32>> {
        if !self.init {
            Err(ManagerError::NotInit)
        } else if let Some(value) = self.locales.get(&locale_id) {
            Ok(value.clone())
        } else {
            Err(ManagerError::NotBound(locale_id))
        }
    }

    pub fn get_all(&self) -> Result<Vec<Locale>, crate::manager::ManagerError<i32>> {
        if !self.init {
            Err(ManagerError::NotInit)
        } else {
            Ok(self.locales.clone().into_values().collect())
        }
    }

}