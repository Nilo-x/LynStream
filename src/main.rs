mod handlers;
mod storage_stream;

use axum::{Router, routing::delete, routing::get, routing::put};
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::broadcast;

use handlers::{handle_delete, handle_read, handle_write};

type Storage = Arc<RwLock<HashMap<String, (Vec<u8>, broadcast::Sender<Bytes>)>>>;

#[tokio::main]
async fn main() {
    println!("Starting http-optimized-storage-server");

    let storage: Storage = Arc::new(RwLock::new(HashMap::new()));

    let app = Router::new()
        .route(
            "/",
            get(|| async { "This is my optimized video http server and we are live baby!" }),
        )
        .route("/read", get(handle_read))
        .route("/{*path}", put(handle_write))
        .route("/delete", delete(handle_delete))
        .with_state(storage);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
