use axum::{
    body::Body,
    extract::{Path, State},
    http::{Response, StatusCode, header},
    response::IntoResponse,
};

use crate::{app::AppState, error::AppError};

pub async fn handle_read(
    State(state): State<AppState>,
    Path(path): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = state.lock().await;
    match service.download(&path).await? {
        Some(bytes) => Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "video/mp4")
            .body(Body::from(bytes))
            .unwrap()),
        None => Err(AppError::NotFound),
    }
}
