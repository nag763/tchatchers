// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! This gathers the list of API that can be used to upload pictures.

use crate::extractor::JwtUserExtractor;
use axum::{body::Bytes, http::StatusCode, response::IntoResponse};
use tchatchers_core::api_response::ApiGenericResponse;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

/// Uploads a picture in the server's static directory.
///
/// The picture will be renamed randomly in order to avoid collisions
///
/// # Arguments
///
/// - body : the picture content.
pub async fn upload_pfp(
    JwtUserExtractor(_): JwtUserExtractor,
    body: Bytes,
) -> Result<impl IntoResponse, ApiGenericResponse> {
    let file_name = Uuid::new_v4().to_string();
    let rel_file_path = format!("./static/{}", file_name);
    let served_file_path: String = format!("/static/{}", file_name);
    let mut file = File::create(&rel_file_path).await?;
    file.write_all(&body).await?;
    Ok((StatusCode::OK, served_file_path))
}
