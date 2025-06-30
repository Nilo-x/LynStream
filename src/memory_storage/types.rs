use bytes::Bytes;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tokio::sync::{RwLock, broadcast};

#[derive(Clone)]
pub struct VideoStream {
    pub data: Vec<u8>,
    pub sender: broadcast::Sender<Bytes>,
    pub is_complete: Arc<AtomicBool>,
}

impl VideoStream {
    pub fn new(
        data: Vec<u8>,
        sender: broadcast::Sender<Bytes>,
    ) -> Self {
        Self {
            data,
            sender,
            is_complete: Arc::new(AtomicBool::new(false)),
        }
    }
}

pub type Storage = Arc<RwLock<HashMap<String, VideoStream>>>;
