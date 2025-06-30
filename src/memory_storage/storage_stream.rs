use bytes::Bytes;
use futures_util::Stream;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

pub struct StorageStream {
    stored_data: Vec<u8>,
    stored_position: usize,
    live_stream: BroadcastStream<Bytes>,
    is_upload_complete: Arc<AtomicBool>,
    finished: bool,
}

impl StorageStream {
    pub fn new(
        stored_data: Vec<u8>,
        live_receiver: broadcast::Receiver<Bytes>,
        is_upload_complete: Arc<AtomicBool>,
    ) -> Self {
        Self {
            stored_data,
            stored_position: 0,
            live_stream: BroadcastStream::new(live_receiver),
            finished: false,
            is_upload_complete,
        }
    }
}

impl Stream for StorageStream {
    type Item = Result<Bytes, std::io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.stored_position < self.stored_data.len() {
            let chunk_size = std::cmp::min(8192, self.stored_data.len() - self.stored_position);
            let end_position = self.stored_position + chunk_size;
            let chunk =
                Bytes::copy_from_slice(&self.stored_data[self.stored_position..end_position]);
            self.stored_position = end_position;
            return Poll::Ready(Some(Ok(chunk)));
        }

        if !self.finished {
            match Pin::new(&mut self.live_stream).poll_next(cx) {
                Poll::Ready(Some(Ok(bytes))) => Poll::Ready(Some(Ok(bytes))),
                Poll::Ready(Some(Err(_))) => {
                    self.finished = true;
                    Poll::Ready(None)
                }
                Poll::Ready(None) => {
                    self.finished = true;
                    Poll::Ready(None)
                }
                Poll::Pending => {
                    if self.is_upload_complete.load(Ordering::Relaxed) {
                        self.finished = true;
                        Poll::Ready(None)
                    } else {
                        Poll::Pending
                    }
                }
            }
        } else {
            Poll::Ready(None)
        }
    }
}
