use crate::error::AppError;
use bytes::Bytes;

pub trait StorageBackend: Send + Sync + 'static {
    fn get(&self, key: &str) -> impl Future<Output = Result<Option<Bytes>, AppError>>;
    fn put(&mut self, key: String, data: Bytes) -> impl Future<Output = Result<(), AppError>>;
    fn delete(&mut self, key: &str) -> impl Future<Output = Result<bool, AppError>>;
    fn list(&self) -> impl Future<Output = Result<Vec<String>, AppError>>;
}

pub mod memory_storage;
pub use memory_storage::MemoryStorage;
