use axum::extract::{Path, Request, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use futures_util::StreamExt;
use std::sync::atomic::Ordering;
use tokio::sync::broadcast;

use crate::memory_storage::types::{Storage, VideoStream};

pub async fn handle_write(
    Path(path): Path<String>,
    State(storage): State<Storage>,
    request: Request,
) -> Result<impl IntoResponse, StatusCode> {
    let path = format!("/{}", path);
    println!("Received http PUT request for path: {}", path);

    let (sender, is_complete) = {
        let mut storage_guard = storage.write().await;
        let stream = storage_guard.entry(path.clone()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(1024);
            VideoStream::new(Vec::new(), tx)
        });
        (stream.sender.clone(), stream.is_complete.clone())
    };

    let mut body_stream = request.into_body().into_data_stream();
    let mut total_bytes = 0;

    while let Some(chunk_result) = StreamExt::next(&mut body_stream).await {
        match chunk_result {
            Ok(chunk) => {
                total_bytes += chunk.len();

                {
                    let mut storage_guard = storage.write().await;
                    if let Some(stream) = storage_guard.get_mut(&path) {
                        stream.data.extend_from_slice(&chunk);
                    }
                }

                let _ = sender.send(chunk);
            }
            Err(_) => return Err(StatusCode::BAD_REQUEST),
        }
    }

    is_complete.store(true, Ordering::Relaxed);
    println!("PUT {} completed: {} bytes", path, total_bytes);
    Ok(format!("Stored {} bytes", total_bytes))
}
