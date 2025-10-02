use axum::{
    Json,
    extract::{Path, State},
};

use crate::{app::AppState, error::AppError};

pub async fn handle_delete(
    State(state): State<AppState>,
    Path(path): Path<String>,
) -> Result<Json<bool>, AppError> {
    let mut svc = state.lock().await;
    let deleted = svc.remove(&path).await?;
    Ok(Json(deleted))
}
