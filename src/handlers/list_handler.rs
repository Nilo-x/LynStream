use crate::memory_storage::types::Storage;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

pub async fn handle_list(State(storage): State<Storage>) -> Result<impl IntoResponse, StatusCode> {
    println!("Received http GET request for list");

    let x = storage.read().await.clone();

    let mut content_list = Vec::new();
    x.iter().for_each(|(k, _)| content_list.push(k.clone()));
    
    Ok(Json(content_list))
}
