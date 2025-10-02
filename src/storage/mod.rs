use std::future::Future;

use crate::error::AppError;
use bytes::Bytes;

pub trait StorageBackend: Send + Sync + 'static {
    fn get(&self, key: &str) -> impl Future<Output = Result<Option<Bytes>, AppError>>;
    fn put(&mut self, key: String, data: Bytes) -> impl Future<Output = Result<(), AppError>>;
    fn delete(&mut self, key: &str) -> impl Future<Output = Result<bool, AppError>>;
    fn list(&self) -> impl Future<Output = Result<Vec<String>, AppError>>;
    fn append(&mut self, key: String, data: Bytes) -> impl Future<Output = Result<(), AppError>>;
    fn subscribe(&self, key: &str) -> impl Future<Output = broadcast::Receiver<Bytes>>;
    fn get_buffer(&self, _key: &str) -> impl std::future::Future<Output = Vec<Bytes>> + Send {
        async { Vec::new() }
    }
}

pub mod memory_storage;
pub use memory_storage::MemoryStorage;
use tokio::sync::broadcast;
