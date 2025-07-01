use crate::memory_storage::types::Storage;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};

pub async fn handle_delete(
    Path(path): Path<String>,
    State(storage): State<Storage>,
) -> Result<impl IntoResponse, StatusCode> {
    let path = format!("/{}", path);
    println!("Received http DELETE request for path: {}", path);

    match storage.write().await.remove(&path) {
        Some(_) => {
            println!("path {} deleted", path);

            let mut response = Response::new(Body::from(path.to_string()));

            response
                .headers_mut()
                .insert("content-type", HeaderValue::from_static("text/plain"));

            Ok(response)
        }
        None => {
            println!("path {} not found", path);
            Err(StatusCode::NOT_FOUND)
        }
    }
}
