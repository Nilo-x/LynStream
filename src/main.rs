use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    println!("Starting http-optimized-storage-server");

    let app = Router::new().route("/", get(|| async { "This is my first server endpoint" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
