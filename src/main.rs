mod storage_stream;

use axum::{
    routing::delete,
    routing::get,
    routing::put,
    Router,
};
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::RwLock;

type Storage = Arc<RwLock<HashMap<String, (Vec<u8>, broadcast::Sender<Bytes>)>>>;

#[tokio::main]
async fn main() {
    println!("Starting http-optimized-storage-server");

    let app = Router::new()
        .route("/", get(|| async { "This is my first server endpoint" }))
        .route("/read", get(|| handle_read()))
        .route("/write", put(|| handle_write()))
        .route("/delete", delete(|| handle_delete()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn handle_read() -> &'static str
{
    "This is going to be my read method"
}

async fn handle_write() -> &'static str
{
    println!("This is going to be my write method");
    "This is going to be my write method"
}

async fn handle_delete() -> &'static str
{
    "This is going to be my delete method"
}