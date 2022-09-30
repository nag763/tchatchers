use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::{Extension, Json, Path},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use futures_util::{SinkExt, StreamExt};
use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};
use regex::Regex;
use sqlx_core::postgres::PgPool;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tchatchers_core::jwt::Jwt;
use tchatchers_core::room::Room;
use tchatchers_core::user::{AuthenticableUser, InsertableUser, User};
use tchatchers_core::ws_message::{WsMessage, WsMessageType};
use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref JWT_COOKIE_HEADER: Regex =
        Regex::new(r####"^jwt=(?P<token_val>[a-zA-Z0-9\._-]*)$"####).unwrap();
}

const JWT_PATH: &str = "jwt";

struct State {
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

    let secured_routes = Router::new()
        .route("/ws/:room", get(ws_handler))
        .layer(middleware::from_fn(secure_route));

    let app = Router::new()
        // top since it matches all routes
        .route("/api/create_user", post(create_user))
        .route("/api/login_exists/:login", get(login_exists))
        .route("/api/authenticate", post(authenticate))
        .route("/api/logout", get(logout))
        .route("/api/validate", get(validate))
        .nest("", secured_routes)
        .layer(Extension(shared_state))
        .layer(CookieManagerLayer::new());

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
    let response_status: StatusCode = match User::login_exists(&login, &state.pg_pool).await {
        false => StatusCode::OK,
        true => StatusCode::CONFLICT,
    };
    (response_status, ())
}

async fn logout(cookies: Cookies) -> impl IntoResponse {
    let mut jwt_cookie = Cookie::new(JWT_PATH, "");
    jwt_cookie.set_path("/");
    jwt_cookie.make_removal();
    cookies.add(jwt_cookie);
    (StatusCode::OK, "")
}

async fn validate(cookies: Cookies, Extension(state): Extension<Arc<State>>) -> impl IntoResponse {
    if let Some(cookie) = cookies.get(JWT_PATH) {
        let value = cookie.value();
        match Jwt::deserialize(value, &state.jwt_secret) {
            Ok(_) => (StatusCode::OK, ""),
            Err(_) => (StatusCode::UNAUTHORIZED, "The jwt couldn't be deserialized"),
        }
    } else {
        (StatusCode::UNAUTHORIZED, "The JWT token hasn't been found")
    }
}

async fn authenticate(
    Json(mut user): Json<AuthenticableUser>,
    Extension(state): Extension<Arc<State>>,
    cookies: Cookies,
) -> impl IntoResponse {
    user.password = state.encrypter.encrypt_str_to_base64(&user.password);
    let user = match user.authenticate(&state.pg_pool).await {
        Some(v) => v,
        None => {
            sleep(Duration::from_secs(3)).await;
            return (StatusCode::NOT_FOUND, "We couldn't connect you, please ensure that the login and password are correct before trying again");
        }
    };
    match user.is_authorized {
        true => {
            let jwt = Jwt::from(user);
            let serialized_jwt : String = jwt.serialize(&state.jwt_secret).unwrap();
            let mut jwt_cookie = Cookie::new(JWT_PATH, serialized_jwt);
            jwt_cookie.set_path("/");
            jwt_cookie.make_permanent();
            jwt_cookie.set_secure(true);
            jwt_cookie.set_http_only(false);
            cookies.add(jwt_cookie);
            (StatusCode::OK, "")
        }
        false => (StatusCode::UNAUTHORIZED, "This user's access has been revoked, contact an admin if you believe you should access this service")
    }
}

async fn create_user(
    Json(mut new_user): Json<InsertableUser>,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    if User::login_exists(&new_user.login, &state.pg_pool).await {
        return (
            StatusCode::BAD_REQUEST,
            "A user with a similar login already exists",
        );
    }

    new_user.password = state.encrypter.encrypt_str_to_base64(&new_user.password);

    match new_user.insert(&state.pg_pool).await {
        Ok(_) => (StatusCode::CREATED, "User created with success"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "An error happened"),
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<State>>,
    Path(room): Path<String>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state, room))
}

async fn handle_socket(socket: WebSocket, state: Arc<State>, room: String) {
    let (mut sender, mut receiver) = socket.split();
    let tx = {
        let mut rooms = state.txs.lock().unwrap();
        match rooms.get(&room) {
            Some(v) => v.clone(),
            None => {
                let (tx, _rx) = broadcast::channel(1000);
                rooms.insert(room, tx.clone());
                tx
            }
        }
    };
    let mut rx = tx.subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    //let tx = state.tx.clone();

    // This task will receive messages from client and send them to broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Add username before message.
            match text.as_str() {
                "Ping" => {
                    let _ = tx.send(String::from("Pong"));
                }
                "Pong" => {
                    continue;
                }
                t => {
                    let msg: WsMessage = serde_json::from_str(t).unwrap();
                    if let (Some(jwt), Some(room)) = (msg.jwt, msg.room) {
                        if let Ok(jwt) = Jwt::deserialize(&jwt, &state.jwt_secret) {
                            let user: User = jwt.into();
                            match msg.message_type {
                                WsMessageType::Send => {
                                    let ws_message = WsMessage {
                                        message_type: WsMessageType::Receive,
                                        content: msg.content,
                                        author: Some(user.into()),
                                        room: Some(room.clone()),
                                        ..WsMessage::default()
                                    };
                                    let _ = tx.send(serde_json::to_string(&ws_message).unwrap());

                                    Room::publish_message_in_room(
                                        &mut state.redis_pool.get().unwrap(),
                                        &room,
                                        ws_message.clone(),
                                    );
                                }
                                WsMessageType::RetrieveMessages => {
                                    let msgs = Room::find_messages_in_room(
                                        &mut state.redis_pool.get().unwrap(),
                                        &room,
                                    );
                                    for msg in msgs {
                                        let _ = tx.send(serde_json::to_string(&msg).unwrap());
                                    }
                                    let ws_message = WsMessage {
                                        message_type: WsMessageType::MessagesRetrieved,
                                        author: Some(user.into()),
                                        ..WsMessage::default()
                                    };
                                    let _ = tx.send(serde_json::to_string(&ws_message).unwrap());
                                }
                                _ => {}
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            };
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}

async fn secure_route<B>(req: Request<B>, next: Next<B>) -> Result<Response, impl IntoResponse> {
    let headers = req.headers();
    if let Some(cookie) = headers.get("cookie") {
        let cookie_val = cookie.to_str().map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Couldn't find cookie value",
            )
        })?;
        let captures = JWT_COOKIE_HEADER.captures(cookie_val).ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Regex couldn't find the cookie value",
        ))?;
        let value: &str = captures.name("token_val").unwrap().as_str();
        match Jwt::deserialize(
            value,
            &req.extensions().get::<Arc<State>>().unwrap().jwt_secret,
        ) {
            Ok(_) => Ok(next.run(req).await),
            Err(_) => Err((StatusCode::UNAUTHORIZED, "JWT invalid")),
        }
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            "The route is only for authorized users, please log in.",
        ))
    }
}
