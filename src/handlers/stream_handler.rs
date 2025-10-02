use axum::{
    Json,
    extract::{Path, Request, State},
    response::IntoResponse,
};
use tokio_stream::StreamExt;

use crate::{app::AppState, error::AppError, services::VideoMetadata};

pub async fn handle_stream(
    State(state): State<AppState>,
    Path(name): Path<String>,
    req: Request,
) -> Result<impl IntoResponse, AppError> {
    let mut body = req.into_body().into_data_stream();
    let mut total_bytes = 0usize;

    while let Some(chunk_result) = body.next().await {
        let chunk = chunk_result.map_err(|err| AppError::BadRequest(err.to_string()))?;
        total_bytes += chunk.len();

        {
            let mut service = state.lock().await;
            service.append_chunk(&name, chunk).await?;
        }
    }

    Ok(Json(VideoMetadata {
        name,
        size: total_bytes,
    }))
}
