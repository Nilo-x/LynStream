use bytes::Bytes;

use crate::{error::AppError, storage::StorageBackend};

pub struct VideoMetadata {
    pub name: String,
    pub size: usize,
}

pub struct VideoService<S: StorageBackend> {
    storage: S,
}

impl<S: StorageBackend> VideoService<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    pub async fn download(&self, name: &str) -> Result<Option<Bytes>, AppError> {
        self.storage.get(name).await
    }

    pub async fn upload(&mut self, name: &str, data: Bytes) -> Result<VideoMetadata, AppError> {
        let size = data.len();
        self.storage.put(name.to_string(), data).await?;
        Ok(VideoMetadata {
            name: name.to_string(),
            size,
        })
    }

    pub async fn remove(&mut self, name: &str) -> Result<bool, AppError> {
        self.storage.delete(name).await
    }

    pub async fn list(&self) -> Result<Vec<String>, AppError> {
        self.storage.list().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryStorage;
    use bytes::Bytes;

    #[tokio::test]
    async fn upload_and_get_video() {
        let storage = MemoryStorage::new();
        let mut service = VideoService::new(storage);

        let data = Bytes::from("hello world");
        service.upload("test.mp4", data.clone()).await.unwrap();

        let retrieved = service.download("test.mp4").await.unwrap();
        assert_eq!(retrieved.unwrap(), data);
    }

    #[tokio::test]
    async fn delete_video() {
        let storage = MemoryStorage::new();
        let mut service = VideoService::new(storage);

        service.upload("foo.mp4", Bytes::from("bar")).await.unwrap();
        let deleted = service.remove("foo.mp4").await.unwrap();

        assert!(deleted);
        assert!(service.download("foo.mp4").await.unwrap().is_none());
    }

    #[tokio::test]
    async fn list_videos() {
        let storage = MemoryStorage::new();
        let mut service = VideoService::new(storage);

        service.upload("a.mp4", Bytes::from("aaa")).await.unwrap();
        service.upload("b.mp4", Bytes::from("bbb")).await.unwrap();

        let mut videos = service.list().await.unwrap();
        videos.sort();
        assert_eq!(videos, vec!["a.mp4", "b.mp4"]);
    }
}
