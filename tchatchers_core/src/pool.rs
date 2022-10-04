//! Crate used to connect to the different services used by the back-end app.
//!
//! The connection are with pools for more efficiency. It uses the user env to
//! connect to the different services, so ensure it is configured before running
//! the application.

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

/// Returns a postgres pool from the user env.
pub async fn get_pg_pool() -> PgPool {
    let connect_options = PgConnectOptions::new()
        .host("pg.tchatche.rs")
        .database(&std::env::var("POSTGRES_DB").expect("No schema defined in .env"))
        .username(&std::env::var("POSTGRES_USER").expect("No user defined in .env"))
        .password(&std::env::var("POSTGRES_PASSWORD").expect("No password defined in .env"));
    PgPoolOptions::new()
        .max_connections(15)
        .connect_with(connect_options)
        .await
        .unwrap()
}

/// Returns a redis pool from the user env.
pub async fn get_redis_pool() -> r2d2::Pool<redis::Client> {
    let client = redis::Client::open("redis://redis.tchatche.rs/").unwrap();
    r2d2::Pool::new(client).unwrap()
}
