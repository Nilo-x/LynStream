use axum::extract::{Path, Request, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use futures_util::StreamExt;
use tokio::sync::broadcast;

use crate::memory_storage::types::Storage;

pub async fn handle_write(
    Path(path): Path<String>,
    State(storage): State<Storage>,
    request: Request
) -> Result<impl IntoResponse, StatusCode>
{
    let path = format!("/{}", path);
    println!("Received http PUT request for path: {}", path);

    let sender = {
        let mut storage_guard = storage.write().await;
        let (_, sender) = storage_guard.entry(path.clone()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(1024);
            (Vec::new(), tx)
        });
        sender.clone()
    };

    let mut body_stream = request.into_body().into_data_stream();
    let mut total_bytes = 0;

    while let Some(chunk_result) = StreamExt::next(&mut body_stream).await {
        match chunk_result {
            Ok(chunk) =>{
                total_bytes += chunk.len();

                {
                    let mut storage_guard = storage.write().await;
                    if let Some((data_vec,_)) = storage_guard.get_mut(&path) {
                        data_vec.extend_from_slice(&chunk);
                    }
                }

                let _ = sender.send(chunk);
            }
            Err(_) => return Err(StatusCode::BAD_REQUEST),
        }
    }

    println!("PUT {} completed: {} bytes", path, total_bytes);
    Ok(format!("Stored {} bytes", total_bytes))
}