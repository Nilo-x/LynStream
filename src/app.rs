use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, put},
};

use tokio::sync::Mutex;

use crate::{
    handlers::{
        handle_delete, handle_join, handle_list, handle_read, handle_stream, handle_upload,
    },
    services::VideoService,
    storage::MemoryStorage,
};

pub type AppState = Arc<Mutex<VideoService<MemoryStorage>>>;

pub fn build_app() -> Router {
    let storage = MemoryStorage::new();
    let service = VideoService::new(storage);
    let state = Arc::new(Mutex::new(service));

    Router::new()
        .route("/stream/{*path}", put(handle_stream))
        .route("/watch/{*path}", get(handle_join))
        .route("/list", get(handle_list))
        .route("/{*path}", get(handle_read))
        .route("/{*path}", put(handle_upload))
        .route("/{*path}", delete(handle_delete))
        .with_state(state)
}
