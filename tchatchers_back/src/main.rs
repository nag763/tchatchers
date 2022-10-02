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
use ws::ws_handler;

#[macro_use]
extern crate lazy_static;

const JWT_PATH: &str = "jwt";

pub struct State {
    encrypter: MagicCrypt256,
    jwt_secret: String,
    txs: Mutex<HashMap<String, broadcast::Sender<String>>>,
    pg_pool: PgPool,
    redis_pool: r2d2::Pool<redis::Client>,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
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
        .layer(CookieManagerLayer::new());

    // run it with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn static_file(
    JwtUserExtractor { jwt: _jwt }: JwtUserExtractor,
    Path(path): Path<String>,
) -> impl IntoResponse {
    match tokio::fs::read(format!("./static/{}", &path)).await {
        Ok(data) => (StatusCode::OK, data),
        Err(_e) => (StatusCode::NOT_FOUND, vec![]),
    }
}
