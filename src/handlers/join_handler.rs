use crate::{app::AppState, error::AppError};
use async_stream::stream;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{Response, StatusCode, header},
    response::IntoResponse,
};
use std::io;
use tokio::sync::broadcast::error;

pub async fn handle_join(
    State(state): State<AppState>,
    Path(path): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let (mut rx, buffer) = {
        let service = state.lock().await;
        let rx = service.subscribe(&path).await;
        let buf = service.get_buffer(&path).await;
        (rx, buf)
    };

    let stream = stream! {

        for chunk in buffer {
            yield Ok::<_, io::Error>(chunk);
        }

        loop {
            match rx.recv().await {
                Ok(chunk) => yield Ok::<_, io::Error>(chunk),
                Err(error::RecvError::Closed) => break,
                Err(error::RecvError::Lagged(_)) => continue,
            }
        }
    };

    let body = Body::from_stream(stream);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "video/mp2t")
        .header(header::TRANSFER_ENCODING, "chunked")
        .body(body)
        .unwrap())
}
