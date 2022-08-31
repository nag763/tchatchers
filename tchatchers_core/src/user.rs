use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryResult;
use sqlx::FromRow;
use sqlx::PgPool;

#[derive(Serialize, Deserialize, FromRow, Debug, Default)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub password: String,
    pub is_authorized: bool,
    pub name: String,
}

impl User {
    pub async fn insert(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        let result = sqlx::query("INSERT INTO CHATTER(login, password, name) VALUES ($1,$2,$3)")
            .bind(&self.login)
            .bind(&self.password)
            .bind(&self.name)
            .execute(pool)
            .await;
        result
    }

    pub async fn find_by_id(id: i32, pool: &PgPool) -> Option<Self> {
        sqlx::query_as("SELECT * FROM CHATTER WHERE id=$1")
            .bind(id)
            .fetch_optional(pool)
            .await
            .unwrap()
    }

    pub async fn find_all(pool: &PgPool) -> Vec<Self> {
        sqlx::query_as("SELECT * FROM CHATTER")
            .fetch_all(pool)
            .await
            .unwrap()
    }
}
