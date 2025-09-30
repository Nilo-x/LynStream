use super::StorageBackend;
use crate::error::AppError;

use bytes::Bytes;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct MemoryStorage {
    inner: Arc<RwLock<HashMap<String, Bytes>>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl StorageBackend for MemoryStorage {
    async fn get(&self, key: &str) -> Result<Option<Bytes>, AppError> {
        let data_map = self.inner.read().await;
        Ok(data_map.get(key).cloned())
    }

    async fn put(&mut self, key: String, data: Bytes) -> Result<(), AppError> {
        let mut data_map = self.inner.write().await;
        data_map.insert(key, data);
        Ok(())
    }

    async fn delete(&mut self, key: &str) -> Result<bool, AppError> {
        let mut data_map = self.inner.write().await;
        Ok(data_map.remove(key).is_some())
    }

    async fn list(&self) -> Result<Vec<String>, AppError> {
        let data_map = self.inner.read().await;
        Ok(data_map.keys().cloned().collect())
    }
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
