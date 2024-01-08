//! Crate used to connect to the different services used by the back-end app.
//!
//! The connection are with pools for more efficiency. It uses the user env to
//! connect to the different services, so ensure it is configured before running
//! the application.

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use std::str::FromStr;

#[cfg(any(feature = "back", feature = "async", feature = "cli"))]
use bb8_redis::{
    bb8::{self, Pool},
    RedisConnectionManager,
};
use log::LevelFilter;
use sqlx::postgres::PgPoolOptions;
use sqlx::ConnectOptions;
use sqlx::PgPool;

/// Returns a postgres pool from the user env.
pub async fn get_pg_pool() -> Result<PgPool, sqlx::Error> {
    let connect_options = sqlx::postgres::PgConnectOptions::new()
        .host(&std::env::var("POSTGRES_HOST").expect("No host defined in .env"))
        .port(
            std::env::var("POSTGRES_PORT")
                .expect("No port defined in .env")
                .parse::<u16>()
                .expect("The port defined in the .env is not valid"),
        )
        .database(&std::env::var("POSTGRES_DB").expect("No schema defined in .env"))
        .username(&std::env::var("POSTGRES_USER").expect("No user defined in .env"))
        .password(&std::env::var("POSTGRES_PASSWORD").expect("No password defined in .env"));
    let connect_options = match std::env::var("SQLX_LOG").ok() {
        Some(v) => connect_options
            .log_statements(LevelFilter::from_str(&v).unwrap_or(LevelFilter::Error))
            .clone(),
        None => connect_options.disable_statement_logging().clone(),
    };
    PgPoolOptions::new()
        .max_connections(15)
        .connect_with(connect_options)
        .await
}

#[cfg(any(feature = "back", feature = "cli"))]
pub async fn get_session_pool() -> Result<Pool<RedisConnectionManager>, redis::RedisError> {
    let redis_host = std::env::var("REDIS_HOST").expect("No redis host defined in .env");
    let redis_port = std::env::var("REDIS_PORT").expect("No redis port defined in .env");
    let client =
        bb8_redis::RedisConnectionManager::new(format!("redis://{redis_host}:{redis_port}/1"))?;
    bb8::Pool::builder().max_size(15).build(client).await
}

#[cfg(any(feature = "back", feature = "async", feature = "cli"))]
pub async fn get_async_pool() -> Result<Pool<RedisConnectionManager>, redis::RedisError> {
    let redis_host = std::env::var("REDIS_HOST").expect("No redis host defined in .env");
    let redis_port = std::env::var("REDIS_PORT").expect("No redis port defined in .env");
    let client =
        bb8_redis::RedisConnectionManager::new(format!("redis://{redis_host}:{redis_port}/2"))?;
    bb8::Pool::builder().max_size(15).build(client).await
}

#[cfg(any(feature = "back", feature = "async", feature = "cli"))]
pub async fn get_token_pool() -> Result<Pool<RedisConnectionManager>, redis::RedisError> {
    let redis_host = std::env::var("REDIS_HOST").expect("No redis host defined in .env");
    let redis_port = std::env::var("REDIS_PORT").expect("No redis port defined in .env");
    let client =
        bb8_redis::RedisConnectionManager::new(format!("redis://{redis_host}:{redis_port}/3"))?;
    bb8::Pool::builder().max_size(15).build(client).await
}
