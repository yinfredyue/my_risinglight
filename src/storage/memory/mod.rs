mod table;

use crate::catalog::GlobalTableId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use table::InMemoryTable;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("duplicate table")]
    DuplicateTable,
    #[error("nonexisting table")]
    NonexistingTable,
}

struct Inner {
    tables: HashMap<GlobalTableId, Arc<InMemoryTable>>,
}

pub struct InMemoryStorage {
    inner: Mutex<Inner>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        let inner = Inner {
            tables: HashMap::new(),
        };
        Self {
            inner: Mutex::new(inner),
        }
    }

    pub fn add_table(&self, id: GlobalTableId) -> Result<(), StorageError> {
        let mut inner = self.inner.lock().unwrap();

        let existing = inner.tables.insert(id, Arc::new(InMemoryTable::new()));
        match existing {
            None => Ok(()),
            Some(_) => Err(StorageError::DuplicateTable),
        }
    }

    pub fn get_table(&self, id: GlobalTableId) -> Result<Arc<InMemoryTable>, StorageError> {
        let inner = self.inner.lock().unwrap();

        let res = inner.tables.get(&id);
        match res {
            None => Err(StorageError::NonexistingTable),
            Some(table) => Ok(Arc::clone(table)),
        }
    }
}
