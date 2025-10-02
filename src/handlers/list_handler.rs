use axum::{Json, extract::State};

use crate::{app::AppState, error::AppError};

pub async fn handle_list(State(state): State<AppState>) -> Result<Json<Vec<String>>, AppError> {
    let svc = state.lock().await;
    let keys = svc.list().await?;
    Ok(Json(keys))
}
