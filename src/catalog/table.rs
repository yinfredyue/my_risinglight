use crate::catalog::column::{ColumnCatalog, ColumnDesc};
use crate::catalog::{ColumnId, TableId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct Inner {
    id: TableId,
    name: String,
    columns: HashMap<ColumnId, Arc<ColumnCatalog>>,
}

pub struct TableCatalog {
    inner: Mutex<Inner>,
}

impl TableCatalog {
    pub fn new(id: TableId, name: String, columns: &[(String, ColumnDesc)]) -> Self {
        TableCatalog {
            inner: Mutex::new(Inner {
                id,
                name,
                columns: HashMap::new(),
            }),
        }
    }

    pub fn id(&self) -> TableId {
        self.inner.lock().unwrap().id
    }

    pub fn name(&self) -> String {
        self.inner.lock().unwrap().name.clone()
    }

    pub fn get_column(&self, id: ColumnId) -> Option<Arc<ColumnCatalog>> {
        self.inner.lock().unwrap().columns.get(&id).cloned()
    }

    pub fn all_columns(&self) -> Vec<Arc<ColumnCatalog>> {
        self.inner
            .lock()
            .unwrap()
            .columns
            .values()
            .cloned()
            .collect()
    }
}
