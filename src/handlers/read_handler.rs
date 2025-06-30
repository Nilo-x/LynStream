use crate::memory_storage::StorageStream;
use crate::memory_storage::types::Storage;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub async fn handle_read(
    Path(path): Path<String>,
    State(storage): State<Storage>,
) -> Result<impl IntoResponse, StatusCode> {
    let path = format!("/{}", path);
    println!("Received http GET request for path: {}", path);

    let (stored_data, live_receiver, is_complete) = {
        let storage_guard = storage.read().await;
        match storage_guard.get(&path) {
            Some(stream) => (
                stream.data.clone(),
                stream.sender.subscribe(),
                stream.is_complete.clone(),
            ),
            None => return Err(StatusCode::NOT_FOUND),
        }
    };

    println!("serving {}: {} bytes stored", path, stored_data.len());

    let video_stream = StorageStream::new(stored_data, live_receiver, is_complete);
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
