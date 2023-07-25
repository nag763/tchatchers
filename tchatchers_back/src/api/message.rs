use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use tchatchers_core::{
    api_response::ApiGenericResponse, profile::Profile, report::Report,
    ws_message::WsMessageContent,
};
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
                return Err(ApiGenericResponse::UnsifficentPriviledges);
            }
            None => return Err(ApiGenericResponse::MessageDoesNotExist),
        }
    }
    WsMessageContent::delete_messages(&vec![message_id], &state.pg_pool).await?;
    Ok(ApiGenericResponse::MessageDeleted)
}

/// Report a message.
///
/// # Arguments
///
/// - `message_id`: The ID of the message to report.
///
/// This function reports a message by inserting a new entry in the `REPORTED_MESSAGE` table of the database. The reported message is associated with the user who made the request, as identified by the JWT token extracted from the request.
///
/// If the insertion is successful, the function returns a tuple containing the status code `StatusCode::OK` and a string indicating that the message has been reported.
///
/// If an error occurs during the insertion, the function checks if the error is a database error. If the error code is `23505`, it means that the message has already been reported, and the function returns a tuple with the status code `StatusCode::BAD_REQUEST` and a corresponding error message. Otherwise, it returns a tuple with the status code `StatusCode::INTERNAL_SERVER_ERROR` and a generic error message indicating that an error occurred while reporting the message.
pub async fn report_message(
    JwtUserExtractor(user): JwtUserExtractor,
    Path(message_id): Path<Uuid>,
    state: State<AppState>,
) -> impl IntoResponse {
    match Report::message(user.user_id, &message_id, &state.pg_pool).await {
        Ok(_) => Ok(ApiGenericResponse::MessageReported),
        Err(e) => {
            if let Some(database_err) = e.as_database_error() {
                if let Some(code) = database_err.code() {
                    if code == "23505" {
                        return Err(ApiGenericResponse::MessageAlreadyReported);
                    }
                }
            }
            eprintln!("{}", e);
            Err(ApiGenericResponse::DbError)
        }
    }
}
