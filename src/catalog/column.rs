use sqlparser::{ast::ColumnDef, ast::ColumnOption};

use crate::catalog::ColumnId;
use crate::types::DataType;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct ColumnDesc {
    is_nullable: bool,
    is_primary: bool,
    data_type: DataType,
}

impl ColumnDesc {
    pub fn is_nullable(&self) -> bool {
        self.is_nullable
    }
    pub fn is_primary(&self) -> bool {
        self.is_primary
    }
    pub fn datatype(&self) -> DataType {
        self.data_type.clone()
    }
}

impl From<ColumnDef> for ColumnDesc {
    fn from(def: ColumnDef) -> Self {
        let mut desc = ColumnDesc {
            is_nullable: true,
            is_primary: false,
            data_type: def.data_type,
        };

        for option_def in def.options {
            match option_def.option {
                ColumnOption::NotNull => desc.is_nullable = false,
                ColumnOption::Unique { is_primary } => {
                    desc.is_primary = is_primary;
                    desc.is_nullable = false;
                }
                _ => (),
            };
        }

        desc
    }
}

#[derive(Debug)]
struct ColumnCatalogInner {
    id: ColumnId,
    name: String,
    desc: Arc<ColumnDesc>,
}

#[derive(Debug)]
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
