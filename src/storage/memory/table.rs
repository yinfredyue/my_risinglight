use crate::array::DataChunk;

use super::StorageError;
use std::sync::Mutex;

struct Inner {
    chunks: Vec<DataChunk>,
}

pub struct InMemoryTable {
    inner: Mutex<Inner>,
}

impl InMemoryTable {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(Inner { chunks: vec![] }),
        }
    }

    pub fn append(&self, chunk: DataChunk) -> Result<(), StorageError> {
        Ok(self.inner.lock().unwrap().chunks.push(chunk))
    }

    pub fn all_chunks(&self) -> Result<Vec<DataChunk>, StorageError> {
        Ok(self.inner.lock().unwrap().chunks.clone())
    }
}
