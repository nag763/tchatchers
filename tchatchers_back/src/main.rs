// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! Server of the application
//!
//! It is used to communicate diretchatchers_core/index.htmltly with the front end and be a layer between
//! the file systems (ie the profile pictures), the services (ie Postgres and
//! Redis) and communicate then in a convenient way with the client application.

pub mod api;
pub mod extractor;
pub mod ws;

use api::message::delete_message;
use api::message::report_message;
use api::pfp::*;
use api::user::*;
use axum::http::header::AUTHORIZATION;
use axum::http::header::COOKIE;
use axum::http::header::SEC_WEBSOCKET_PROTOCOL;
use axum::routing::delete;
use axum::routing::get_service;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use bb8_redis::bb8;
use bb8_redis::RedisConnectionManager;
use sqlx_core::postgres::PgPool;
use std::iter::once;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::join;
use tokio::signal::unix::SignalKind;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::request_id::MakeRequestUuid;
use tower_http::sensitive_headers::SetSensitiveRequestHeadersLayer;
use tower_http::services::ServeDir;
use tower_http::trace::DefaultOnFailure;
use tower_http::trace::DefaultOnRequest;
use tower_http::trace::DefaultOnResponse;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tower_http::LatencyUnit;
use tower_http::ServiceBuilderExt;
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use ws::ws_handler;
use ws::WsRooms;

const REFRESH_TOKEN_PATH: &str = "refresh_token";

#[derive(Clone)]
/// The data that is shared across the processes.
pub struct AppState {
    /// The secret to encrypt the JWT.
    jwt_secret: String,
    /// Refresh token secret.
    refresh_token_secret: String,
    /// The WS rooms, with the key being the room name.
    txs: Arc<Mutex<WsRooms>>,
    /// The Postgres pool.
    pg_pool: PgPool,
    /// Redis session pool.
    session_pool: bb8::Pool<RedisConnectionManager>,
    /// Redis async pool.
    async_pool: bb8::Pool<RedisConnectionManager>,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("TOWER_LOG").unwrap_or_else(|_| "tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let jwt_secret = std::env::var("JWT_SECRET").expect("No jwt secret has been defined");
    let refresh_token_secret = std::env::var("REFRESH_TOKEN_SECRET")
        .expect("No refresh token signature key has been defined");
    let (pg_pool, session_pool, async_pool) = join!(
        tchatchers_core::pool::get_pg_pool(),
        tchatchers_core::pool::get_session_pool(),
        tchatchers_core::pool::get_async_pool()
    );

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Could not apply migrations on the database");
    let shared_state = AppState {
        refresh_token_secret,
        jwt_secret,
        txs: Arc::new(Mutex::new(WsRooms::default())),
        pg_pool,
        session_pool,
        async_pool,
    };

    let app = Router::new()
        .route(
            "/api/user",
            post(create_user).put(update_user).delete(delete_user),
        )
        .route("/api/login_exists/:login", get(login_exists))
        .route("/api/user/revoke/:user_id", post(revoke_user))
        .route("/api/user/:reported_user/report", post(report_user))
        .route(
            "/api/authenticate",
            post(authenticate).patch(reauthenticate),
        )
        .route("/api/logout", get(logout))
        .route("/api/validate", get(validate))
        .route("/api/pfp", post(upload_pfp))
        .route("/api/whoami", get(whoami))
        .route("/api/message/:message_id", delete(delete_message))
        .route("/api/message/:message_id/report", post(report_message))
        .route("/ws/:room", get(ws_handler))
        .nest_service(
            "/static",
            get_service(ServeDir::new("static"))
                .handle_error(|_| async { (StatusCode::NOT_FOUND, "File not found") }),
        )
        .with_state(shared_state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                )
                .on_failure(DefaultOnFailure::new().level(Level::ERROR)),
        )
        .layer(SetSensitiveRequestHeadersLayer::new(once(COOKIE)))
        .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
        .layer(SetSensitiveRequestHeadersLayer::new(once(
            SEC_WEBSOCKET_PROTOCOL,
        )))
        .layer(
            ServiceBuilder::new()
                .set_x_request_id(MakeRequestUuid)
                .propagate_x_request_id(),
        );

    // run it with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate()).unwrap();
            let mut sigkill = tokio::signal::unix::signal(SignalKind::interrupt()).unwrap();
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {},
                _ = sigterm.recv() => {},
                _ = sigkill.recv() => {},
            }
            println!("Shutting down...");
        })
        .await
        .unwrap();
}
