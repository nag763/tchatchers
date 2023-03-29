//! Crate used to connect to the different services used by the back-end app.
//!
//! The connection are with pools for more efficiency. It uses the user env to
//! connect to the different services, so ensure it is configured before running
//! the application.

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use std::str::FromStr;

use log::LevelFilter;
use r2d2::Pool;
use redis::Client;
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgPoolOptions;
use sqlx::ConnectOptions;
use sqlx::PgPool;

/// Returns a postgres pool from the user env.
pub async fn get_pg_pool() -> PgPool {
    let mut connect_options = PgConnectOptions::new()
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
        .unwrap()
}

pub fn get_redis_pool() -> Pool<Client> {
    let redis_host = std::env::var("REDIS_HOST").expect("No redis host defined in .env");
    let redis_port = std::env::var("REDIS_PORT").expect("No redis port defined in .env");
    let client = redis::Client::open(format!("redis://{redis_host}:{redis_port}")).unwrap();
    r2d2::Pool::builder().max_size(15).build(client).unwrap()
}
