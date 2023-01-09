use serde::{Deserialize, Serialize};

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
    pub(crate) async fn get_all(pool: &sqlx::PgPool) -> Vec<Locale> {
        sqlx::query_as("SELECT * FROM LOCALE")
            .fetch_all(pool)
            .await
            .unwrap()
    }

    pub(crate) async fn get_all_sorted_by_name(pool: &sqlx::PgPool) -> Vec<Locale> {
        sqlx::query_as("SELECT * FROM LOCALE ORDER BY long_name")
            .fetch_all(pool)
            .await
            .unwrap()
    }
}

#[cfg(feature = "back")]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LocaleManager {
    locales: std::collections::HashMap<i32, Locale>,
    init: bool,
}

#[cfg(feature = "back")]
impl LocaleManager {
    pub async fn init(pg_pool: &sqlx::PgPool) -> LocaleManager {
        let locales: std::collections::HashMap<i32, Locale> =
            Locale::get_all_sorted_by_name(pg_pool)
                .await
                .iter()
                .map(|l| (l.id, l.clone()))
                .collect();
        LocaleManager {
            locales,
            init: true,
        }
    }

    pub fn get(&self, locale_id: i32) -> Result<Locale, crate::manager::ManagerError<i32>> {
        if !self.init {
            Err(crate::manager::ManagerError::NotInit)
        } else if let Some(value) = self.locales.get(&locale_id) {
            Ok(value.clone())
        } else {
            Err(crate::manager::ManagerError::NotBound(locale_id))
        }
    }

    pub fn get_all(&self) -> Result<Vec<Locale>, crate::manager::ManagerError<i32>> {
        if !self.init {
            Err(crate::manager::ManagerError::NotInit)
        } else {
            Ok(self.locales.clone().into_values().collect())
        }
    }
}
