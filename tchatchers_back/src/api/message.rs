use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use tchatchers_core::{profile::Profile, ws_message::WsMessageContent};
use uuid::Uuid;

use crate::{extractor::JwtUserExtractor, AppState};

/// Delete a message
///
/// This endpoint allows to delete a message in the DB.
///
/// # Arguments
///
/// - message_id : the message's uuid to delete.
pub async fn delete_message(
    JwtUserExtractor(user): JwtUserExtractor,
    Path(message_id): Path<Uuid>,
    state: State<AppState>,
) -> impl IntoResponse {
    if user.user_profile == Profile::User {
        match WsMessageContent::get_one(&message_id, &state.pg_pool).await {
            Some(message) if message.author.id == user.user_id => (),
            Some(_) => {
                return (
                    StatusCode::FORBIDDEN,
                    "The user can only delete his own requests",
                )
            }
            None => return (StatusCode::NOT_FOUND, "This message doesn't exist"),
        }
    }
    match WsMessageContent::delete_messages(&vec![message_id], &state.pg_pool).await {
        Ok(_) => (StatusCode::OK, "Message deleted"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "An error happened"),
    }
}
