use axum::{
    Json,
    extract::{Path, State},
};
use bytes::Bytes;

use crate::{app::AppState, error::AppError, services::VideoMetadata};

pub async fn handle_upload(
    State(state): State<AppState>,
    Path(path): Path<String>,
    body: Bytes,
) -> Result<Json<VideoMetadata>, AppError> {
    let mut service = state.lock().await;
    let meta = service.upload(&path, body).await?;
    Ok(Json(meta))
}
