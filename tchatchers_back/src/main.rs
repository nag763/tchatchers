//! Server of the application
//!
//! It is used to communicate directly with the front end and be a layer between
//! the file systems (ie the profile pictures), the services (ie Postgres and
//! Redis) and communicate then in a convenient way with the client application.

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

pub mod api;
pub mod extractor;
pub mod validator;
pub mod ws;

use api::admin::translation::get_all_translations;
use api::admin::translation::get_translations_for_locale;
use api::admin::translation::reload_translations;
use api::app_context::app_context;
use api::locale::get_locale_id;
use api::locale::get_locales;
use api::pfp::*;
use api::user::*;
use axum::routing::get_service;
use axum::routing::put;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use sqlx_core::postgres::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;
use tchatchers_core::navlink::NavlinkManager;
use tchatchers_core::translation::TranslationManager;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use ws::ws_handler;
use ws::WsRooms;

const JWT_PATH: &str = "jwt";

/// The data that is shared across the processes.
pub struct AppState {
    /// The secret to encrypt the JWT.
    jwt_secret: String,
    /// The WS rooms, with the key being the room name.
    txs: Mutex<WsRooms>,
    /// The Postgres pool.
    pg_pool: PgPool,
    translation_manager: Mutex<TranslationManager>,
    navlink_manager: Mutex<NavlinkManager>,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "tchatchers_back=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let jwt_secret = std::env::var("JWT_SECRET").expect("No jwt secret has been defined");
    let pg_pool = tchatchers_core::pool::get_pg_pool().await;
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Could not apply migrations on the database");
    let shared_state = Arc::new(AppState {
        navlink_manager: Mutex::new(NavlinkManager::init(&pg_pool).await),
        translation_manager: Mutex::new(TranslationManager::init(&pg_pool).await),
        jwt_secret,
        txs: Mutex::new(WsRooms::default()),
        pg_pool,
    });

    let app = Router::new()
        .route(
            "/api/user",
            post(create_user).put(update_user).delete(delete_user),
        )
        .route("/api/login_exists/:login", get(login_exists))
        .route("/api/authenticate", post(authenticate))
        .route("/api/logout", get(logout))
        .route("/api/validate", get(validate))
        .route("/api/pfp", post(upload_pfp))
        .route("/api/app_context", get(app_context))
        .route("/api/locale/", get(get_locales))
        .route("/api/locale/:locale_id", get(get_locale_id))
        .route(
            "/api/admin/translation",
            put(reload_translations).get(get_all_translations),
        )
        .route(
            "/api/admin/translation/:locale_id",
            get(get_translations_for_locale),
        )
        .route("/ws/:room", get(ws_handler))
        .nest_service(
            "/static",
            get_service(ServeDir::new("static"))
                .handle_error(|_| async { (StatusCode::NOT_FOUND, "File not found") }),
        )
        .with_state(shared_state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    // run it with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
