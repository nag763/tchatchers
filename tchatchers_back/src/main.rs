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
use std::sync::{Arc};
use tchatchers_core::user::{AuthenticableUser, InsertableUser, User};
use tokio::time::{sleep, Duration};
use tokio::sync::broadcast;
use futures_util::{StreamExt, SinkExt};

struct State {
    encrypter: MagicCrypt256,
    tx: broadcast::Sender<String>,
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let secret = std::env::var("SECRET").expect("No secret has been defined");
    let encrypter = new_magic_crypt!(&secret, 256);
    let (tx, _rx) = broadcast::channel(100);
    let shared_state = Arc::new(State {
        encrypter,
        pool: tchatchers_core::pool::get_pool().await,
        tx
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

async fn ws_handler(ws: WebSocketUpgrade, Extension(state): Extension<Arc<State>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<State>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let tx = state.tx.clone();

    
    // This task will receive messages from client and send them to broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Add username before message.
            let ret = match text.as_str() {
                "Ping" => "Pong",
                "Pong" => {break;}
                t => t
            };
            let _ = tx.send(String::from(ret));
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };


}
