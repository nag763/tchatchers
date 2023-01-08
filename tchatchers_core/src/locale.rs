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
    pub async fn get_all(pool: &sqlx::PgPool) -> Vec<Locale> {
        sqlx::query_as("SELECT * FROM LOCALE")
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
