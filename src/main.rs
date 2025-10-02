use http_video_storage_server::app::build_app;

#[tokio::main]
async fn main() {
    let app = build_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Livestream capable server has been started. :)");
    axum::serve(listener, app).await.unwrap()
}
