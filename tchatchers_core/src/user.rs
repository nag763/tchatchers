use crate::jwt::Jwt;
use serde::{Deserialize, Serialize};
#[cfg(feature = "back")]
use sqlx::postgres::PgQueryResult;
#[cfg(feature = "back")]
use sqlx::FromRow;
#[cfg(feature = "back")]
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug, Default)]
#[cfg_attr(feature = "back", derive(FromRow))]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub login: String,
    pub password: String,
    pub is_authorized: bool,
    pub name: String,
    pub pfp: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct PartialUser {
    pub id: i32,
    pub login: String,
    pub name: String,
    pub pfp: Option<String>,
}

impl From<User> for PartialUser {
    fn from(user: User) -> PartialUser {
        PartialUser {
            id: user.id,
            login: user.login,
            name: user.name,
            pfp: user.pfp,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdatableUser {
    pub id: i32,
    pub name: String,
    pub pfp: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
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

impl From<Jwt> for User {
    fn from(jwt: Jwt) -> User {
        User {
            id: jwt.user.id,
            login: jwt.user.login,
            name: jwt.user.name,
            ..User::default()
        }
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

#[cfg(feature = "back")]
impl UpdatableUser {
    pub async fn update(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query("UPDATE CHATTER SET name=$1, pfp=$2 WHERE id=$3")
            .bind(&self.name)
            .bind(&self.pfp)
            .bind(&self.id)
            .execute(pool)
            .await
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AuthenticableUser {
    pub login: String,
    pub password: String,
}

#[cfg(feature = "back")]
impl AuthenticableUser {
    pub async fn authenticate(&self, pool: &PgPool) -> Option<User> {
        sqlx::query_as("SELECT * FROM CHATTER WHERE login=$1 AND password=$2")
            .bind(&self.login)
            .bind(&self.password)
            .fetch_optional(pool)
            .await
            .unwrap()
    }
}
