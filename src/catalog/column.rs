use crate::catalog::ColumnId;
use crate::types::DataType;
use std::sync::{Arc, Mutex};

struct ColumnDescInner {
    is_nullable: bool,
    is_primary: bool,
    data_type: DataType,
}

pub struct ColumnDesc {
    inner: Mutex<ColumnDescInner>,
}

impl ColumnDesc {
    pub fn is_nullable(&self) -> bool {
        self.inner.lock().unwrap().is_nullable
    }
    pub fn is_primary(&self) -> bool {
        self.inner.lock().unwrap().is_primary
    }
    pub fn datatype(&self) -> DataType {
        self.inner.lock().unwrap().data_type.clone()
    }
}

struct ColumnCatalogInner {
    id: ColumnId,
    name: String,
    desc: Arc<ColumnDesc>,
}

pub struct ColumnCatalog {
    inner: Mutex<ColumnCatalogInner>,
}

impl ColumnCatalog {
    pub fn new(id: ColumnId, name: String, desc: ColumnDesc) -> Self {
        ColumnCatalog {
            inner: Mutex::new(ColumnCatalogInner {
                id,
                name,
                desc: Arc::new(desc),
            }),
        }
    }
    pub fn id(&self) -> ColumnId {
        self.inner.lock().unwrap().id
    }
    pub fn name(&self) -> String {
        self.inner.lock().unwrap().name.clone()
    }
    pub fn desc(&self) -> Arc<ColumnDesc> {
        self.inner.lock().unwrap().desc.clone()
    }
}
