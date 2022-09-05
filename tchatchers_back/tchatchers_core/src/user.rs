use serde::{Deserialize, Serialize};
#[cfg(feature = "back")]
use sqlx::postgres::PgQueryResult;
#[cfg(feature = "back")]
use sqlx::FromRow;
#[cfg(feature = "back")]
use sqlx::PgPool;

#[cfg(feature = "back")]
#[derive(Serialize, Deserialize, FromRow, Debug, Default)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub password: String,
    pub is_authorized: bool,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InsertableUser {
    pub login: String,
    pub password: String,
    pub name: String,
}

#[cfg(feature = "back")]
impl User {
    pub async fn find_by_id(id: i32, pool: &PgPool) -> Option<Self> {
        sqlx::query_as("SELECT * FROM CHATTER WHERE id=$1")
            .bind(id)
            .fetch_optional(pool)
            .await
            .unwrap()
    }

    pub async fn login_exists(login: &str, pool: &PgPool) -> bool {
        let row: (bool,) = sqlx::query_as("SELECT COUNT(id)!=0 FROM CHATTER WHERE login=$1")
            .bind(login)
            .fetch_one(pool)
            .await
            .unwrap();
        row.0
    }

    pub async fn find_all(pool: &PgPool) -> Vec<Self> {
        sqlx::query_as("SELECT * FROM CHATTER")
            .fetch_all(pool)
            .await
            .unwrap()
    }
}

#[cfg(feature = "back")]
impl InsertableUser {
    pub async fn insert(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("INSERT INTO CHATTER(login, password, name) VALUES ($1,$2,$3)")
            .bind(&self.login)
            .bind(&self.password)
            .bind(&self.name)
            .execute(pool)
            .await
    }
}
