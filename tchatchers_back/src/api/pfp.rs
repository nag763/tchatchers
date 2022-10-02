use axum::{body::Bytes, http::StatusCode, response::IntoResponse};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

pub async fn upload_pfp(body: Bytes) -> impl IntoResponse {
    let file_name = format!("/static/{}.jpg", Uuid::new_v4());
    let mut file = File::create(format!(".{}", &file_name)).await.unwrap();
    file.write_all(&body).await.unwrap();
    (StatusCode::OK, file_name)
}
