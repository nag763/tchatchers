/// This gathers the list of API that can be used to upload pictures.

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use crate::extractor::JwtUserExtractor;
use axum::{body::Bytes, http::StatusCode, response::IntoResponse};
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
) -> impl IntoResponse {
    let file_name = format!("/static/{}.jpg", Uuid::new_v4());
    let mut file = File::create(format!(".{}", &file_name)).await.unwrap();
    file.write_all(&body).await.unwrap();
    (StatusCode::OK, file_name)
}
