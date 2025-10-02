use super::StorageBackend;
use crate::error::AppError;

use bytes::{Bytes, BytesMut};
use ringbuffer::{AllocRingBuffer, RingBuffer};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};

const RING_BUFFER_SIZE: usize = 500;

pub struct MemoryStorage {
    inner: Arc<RwLock<HashMap<String, Bytes>>>,
    streams: Arc<RwLock<HashMap<String, broadcast::Sender<Bytes>>>>,
    buffers: Arc<RwLock<HashMap<String, AllocRingBuffer<Bytes>>>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
            streams: Arc::new(RwLock::new(HashMap::new())),
            buffers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn get_or_create_stream(&self, key: &str) -> broadcast::Sender<Bytes> {
        let mut streams = self.streams.write().await;

        streams
            .entry(key.to_string())
            .or_insert_with(|| {
                let (tx, _) = broadcast::channel(1024);
                tx
            })
            .clone()
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl StorageBackend for MemoryStorage {
    async fn get(&self, key: &str) -> Result<Option<Bytes>, AppError> {
        let data_map = self.inner.read().await;
        Ok(data_map.get(key).cloned())
    }

    async fn put(&mut self, key: String, data: Bytes) -> Result<(), AppError> {
        let mut data_map = self.inner.write().await;
        data_map.insert(key.clone(), data.clone());

        let tx = self.get_or_create_stream(&key).await;
        let _ = tx.send(data);

        Ok(())
    }

    async fn delete(&mut self, key: &str) -> Result<bool, AppError> {
        let mut data_map = self.inner.write().await;
        let removed = data_map.remove(key).is_some();

        let mut streams = self.streams.write().await;
        streams.remove(key);

        Ok(removed)
    }

    async fn list(&self) -> Result<Vec<String>, AppError> {
        let data_map = self.inner.read().await;
        Ok(data_map.keys().cloned().collect())
    }

    async fn append(&mut self, key: String, data: Bytes) -> Result<(), AppError> {
        {
            let mut map = self.inner.write().await;
            map.entry(key.clone())
                .and_modify(|existing| {
                    let mut buffer = BytesMut::from(existing.as_ref());
                    buffer.extend_from_slice(&data);
                    *existing = buffer.freeze();
                })
                .or_insert(data.clone());
        }

        {
            let tx = self.get_or_create_stream(&key).await;
            let _ = tx.send(data.clone());
        }

        {
            let mut buffers = self.buffers.write().await;

            if contains_sps_or_idr(&data) {
                buffers.insert(key.clone(), AllocRingBuffer::new(RING_BUFFER_SIZE));
            }

            let buf = buffers
                .entry(key.clone())
                .or_insert_with(|| AllocRingBuffer::new(RING_BUFFER_SIZE));
            buf.enqueue(data);
        }

        Ok(())
    }

    async fn subscribe(&self, key: &str) -> broadcast::Receiver<Bytes> {
        self.get_or_create_stream(key).await.subscribe()
    }

    async fn get_buffer(&self, key: &str) -> Vec<Bytes> {
        let buffers = self.buffers.read().await;
        buffers
            .get(key)
            .map(|rb| rb.iter().cloned().collect())
            .unwrap_or_default()
    }
}

fn contains_sps_or_idr(data: &[u8]) -> bool {
    let len = data.len();
    let mut i = 0;

    while i + 4 < len {
        // H.264 start codes: 0x000001 or 0x00000001
        if data[i..].starts_with(&[0, 0, 1]) {
            let nal_header = data[i + 3];
            let nal_unit_type = nal_header & 0x1F;
            if nal_unit_type == 5 || nal_unit_type == 7 || nal_unit_type == 8 {
                return true;
            }
            i += 3;
        } else if data[i..].starts_with(&[0, 0, 0, 1]) {
            let nal_header = data[i + 4];
            let nal_unit_type = nal_header & 0x1F;
            if nal_unit_type == 5 || nal_unit_type == 7 || nal_unit_type == 8 {
                return true;
            }
            i += 4;
        } else {
            i += 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[tokio::test]
    async fn put_and_get() {
        let mut storage = MemoryStorage::new();
        storage
            .put("foo".to_string(), Bytes::from("bar"))
            .await
            .unwrap();

        let result = storage.get("foo").await.unwrap();
        assert_eq!(result.unwrap(), Bytes::from("bar"));
    }

    #[tokio::test]
    async fn delete_item() {
        let mut storage = MemoryStorage::new();
        storage
            .put("foo".to_string(), Bytes::from("bar"))
            .await
            .unwrap();

        let deleted = storage.delete("foo").await.unwrap();
        assert!(deleted);

        let result = storage.get("foo").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn list_keys() {
        let mut storage = MemoryStorage::new();
        storage
            .put("a.mp4".to_string(), Bytes::from("aaa"))
            .await
            .unwrap();
        storage
            .put("b.mp4".to_string(), Bytes::from("bbb"))
            .await
            .unwrap();

        let mut keys = storage.list().await.unwrap();
        keys.sort();
        assert_eq!(keys, vec!["a.mp4".to_string(), "b.mp4".to_string()]);
    }
}
