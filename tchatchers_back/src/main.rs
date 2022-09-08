use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};
use sqlx_core::postgres::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;
use tchatchers_core::user::{AuthenticableUser, InsertableUser, User};
use tokio::time::{sleep, Duration};

struct State {
    encrypter: MagicCrypt256,
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let secret = std::env::var("SECRET").expect("No secret has been defined");
    let encrypter = new_magic_crypt!(&secret, 256);
    let shared_state = Arc::new(State {
        encrypter,
        pool: tchatchers_core::pool::get_pool().await,
    });

    let app = Router::new()
        // top since it matches all routes
        .route("/ws", get(ws_handler))
        .route("/api/create_user", post(create_user))
        .route("/api/login_exists/:login", get(login_exists))
        .route("/api/authenticate", post(authenticate))
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

async fn authenticate(
    Json(mut user): Json<AuthenticableUser>,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    user.password = state.encrypter.encrypt_str_to_base64(&user.password);
    let user = match user.authenticate(&state.pool).await {
        Some(v) => v,
        None => {
            sleep(Duration::from_secs(3)).await;
            return (StatusCode::NOT_FOUND, "We couldn't connect you, please ensure that the login and password are correct before trying again");
        }
    };
    match user.is_authorized {
        true => (StatusCode::OK, "Foo"),
        false => (StatusCode::UNAUTHORIZED, "This user's access has been revoked, contact an admin if you believe you should access this service")
    }
}

async fn create_user(
    Json(mut new_user): Json<InsertableUser>,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    if User::login_exists(&new_user.login, &state.pool).await {
        return (
            StatusCode::BAD_REQUEST,
            "A user with a similar login already exists",
        );
    }

    new_user.password = state.encrypter.encrypt_str_to_base64(&new_user.password);

    match new_user.insert(&state.pool).await {
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
