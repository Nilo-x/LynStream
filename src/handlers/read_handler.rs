use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::memory_storage::StorageStream;
use crate::memory_storage::types::Storage;

pub async fn handle_read(
    Path(path): Path<String>,
    State(storage): State<Storage>,
) -> Result<impl IntoResponse, StatusCode> {
    let path = format!("/{}", path);
    println!("Received http GET request for path: {}", path);

    let (stored_data, live_receiver) = {
        let storage_guard = storage.read().await;
        match storage_guard.get(&path) {
            Some((data_vec, sender)) => (data_vec.clone(), sender.subscribe()),
            None => return Err(StatusCode::NOT_FOUND),
        }
    };

    println!("serving {}: {} bytes stored", path, stored_data.len());

    let video_stream = StorageStream::new(stored_data, live_receiver);
    let body = Body::from_stream(video_stream);

    let mut response = Response::new(body);

    response
        .headers_mut()
        .insert("content-type", "video/mp4".parse().unwrap());

    response
        .headers_mut()
        .insert("transfer-encoding", "chunked".parse().unwrap());

    Ok(response)
}
