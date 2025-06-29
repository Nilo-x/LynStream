use std::collections::HashMap;
use std::sync::Arc;
use bytes::Bytes;
use tokio::sync::{broadcast, RwLock};

pub type Storage = Arc<RwLock<HashMap<String, (Vec<u8>, broadcast::Sender<Bytes>)>>>;
