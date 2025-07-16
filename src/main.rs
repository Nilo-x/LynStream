mod handlers;
mod memory_storage;

use handlers::{handle_delete, handle_read, handle_write, handle_list};
use memory_storage::types::Storage;

use axum::{Router, routing::delete, routing::get, routing::put};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    println!("Starting http-optimized-storage-server");

    let storage: Storage = Arc::new(RwLock::new(HashMap::new()));
    // TODO: co se deje kdyz uploadnu dvakrat stejnou path?
    // TODO: dodelat any requst handle
    // TODO: mozna front-end treba v tauri ? (asi faze 2.0 projektu)
    let app = Router::new()
        .route(
            "/",
            get(|| async { "This is my optimized video http server and we are live baby!" }),
        )
        .route("/{*path}", get(handle_read))
        .route("/{*path}", put(handle_write))
        .route("/{*path}", delete(handle_delete))
        .route("/list", get(handle_list))
        .with_state(storage);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
