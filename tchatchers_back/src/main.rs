use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use sqlx_core::postgres::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;
use tchatchers_core::user::{InsertableUser, User};

struct State {
    pool: PgPool,
    secret: String
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let shared_state = Arc::new(State {
        pool: tchatchers_core::pool::get_pool().await,
        secret: std::env::var("SECRET").expect("No secret has been defined")
    });

    let app = Router::new()
        // top since it matches all routes
        .route("/ws", get(ws_handler))
        .route("/api/create_user", post(create_user))
        .route("/api/login_exists/:login", get(login_exists))
        .layer(Extension(shared_state));

    // run it with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn login_exists(
    Path(login): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    let response_status: StatusCode = match User::login_exists(&login, &state.pool).await {
        false => StatusCode::OK,
        true => StatusCode::CONFLICT,
    };
    (response_status, ())
}

async fn create_user(
    Json(new_user): Json<InsertableUser>,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    if User::login_exists(&new_user.login, &state.pool).await {
        return (
            StatusCode::BAD_REQUEST,
            "A user with a similar login already exists",
        );
    }
    match new_user.insert(&state.secret, &state.pool).await {
        Ok(_) => (StatusCode::CREATED, "User created with success"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "An error happened"),
    }
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    loop {
        if let Some(msg) = socket.recv().await {
            if let Ok(msg) = msg {
                match msg {
                    Message::Text(t) => {
                        let ret = match t.as_str() {
                            "Ping" => "Pong",
                            "Pong" => {
                                return;
                            }
                            t => t,
                        };
                        socket.send(Message::Text(ret.into())).await.unwrap();
                    }
                    Message::Binary(_) => {
                        println!("client sent binary data");
                    }
                    Message::Ping(_) => {
                        println!("socket ping");
                    }
                    Message::Pong(_) => {
                        println!("socket pong");
                    }
                    Message::Close(_) => {
                        println!("client disconnected");
                        return;
                    }
                }
            } else {
                println!("client disconnected");
                return;
            }
        }
    }
}
