//! Server of the application
//!
//! It is used to communicate directly with the front end and be a layer between
//! the file systems (ie the profile pictures), the services (ie Postgres and 
//! Redis) and communicate then in a convenient way with the client application.

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

pub mod api;
pub mod extractor;
pub mod ws;

use crate::extractor::JwtUserExtractor;
use api::pfp::*;
use api::user::*;
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Router,
};
use magic_crypt::{new_magic_crypt, MagicCrypt256};
use sqlx_core::postgres::PgPool;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use ws::ws_handler;

#[macro_use]
extern crate lazy_static;

const JWT_PATH: &str = "jwt";

/// The data that is shared across the processes.
pub struct State {
    /// The password encryption mechanism.
    encrypter: MagicCrypt256,
    /// The secret to encrypt the JWT.
    jwt_secret: String,
    /// The WS rooms, with the key being the room name.
    txs: Mutex<HashMap<String, broadcast::Sender<String>>>,
    /// The Postgres pool.
    pg_pool: PgPool,
    /// The Redis pool.
    redis_pool: r2d2::Pool<redis::Client>,
}

#[tokio::main]
async fn main() {
    
    dotenv::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_websockets=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pwd_secret = std::env::var("PWD_SECRET").expect("No password secret has been defined");
    let jwt_secret = std::env::var("JWT_SECRET").expect("No jwt secret has been defined");
    let encrypter = new_magic_crypt!(&pwd_secret, 256);
    let shared_state = Arc::new(State {
        encrypter,
        jwt_secret,
        pg_pool: tchatchers_core::pool::get_pg_pool().await,
        redis_pool: tchatchers_core::pool::get_redis_pool().await,
        txs: Mutex::new(HashMap::new()),
    });

    let app = Router::new()
        .route("/api/user", post(create_user))
        .route("/api/login_exists/:login", get(login_exists))
        .route("/api/authenticate", post(authenticate))
        .route("/api/logout", get(logout))
        .route("/api/validate", get(validate))
        .route("/api/user", put(update_user))
        .route("/api/pfp", post(upload_pfp))
        .route("/ws/:room", get(ws_handler))
        .route("/static/:path", get(static_file))
        .layer(Extension(shared_state))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .layer(CookieManagerLayer::new());

    // run it with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Service used to serve the static files.
///
/// Th user must be authenticated to access those files.
///
/// # Argumments
///
/// - jwt : The user authentication token.
/// - path : The path the user would like to access.
async fn static_file(
    JwtUserExtractor(_jwt): JwtUserExtractor,
    Path(path): Path<String>,
) -> impl IntoResponse {
    match tokio::fs::read(format!("./static/{}", &path)).await {
        Ok(data) => (StatusCode::OK, data),
        Err(_e) => (StatusCode::NOT_FOUND, vec![]),
    }
}
