use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tchatchers_core::user::{InsertableUser, User};

#[tokio::main]
async fn main() {
    let app = Router::new()
        // top since it matches all routes
        .route("/ws", get(ws_handler))
        .route("/api/create_user", post(create_user))
        .route("/api/login_exists/:login", get(login_exists));

    // run it with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn login_exists(Path(login): Path<String>) -> impl IntoResponse {
    let pool = tchatchers_core::pool::get_pool().await;
    let response_status: StatusCode = match User::login_exists(&login, &pool).await {
        false => StatusCode::OK,
        true => StatusCode::CONFLICT,
    };
    (response_status, ())
}

async fn create_user(Json(new_user): Json<InsertableUser>) -> impl IntoResponse {
    let pool = tchatchers_core::pool::get_pool().await;
    if User::login_exists(&new_user.login, &pool).await {
        return (
            StatusCode::BAD_REQUEST,
            "A user with a similar login already exists",
        );
    }
    match new_user.insert(&pool).await {
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
