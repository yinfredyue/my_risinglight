use crate::catalog::column::{ColumnCatalog, ColumnDesc};
use crate::catalog::{ColumnId, TableId};
use crate::utils::id_gen::IntIdGen;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::CatalogError;

#[derive(Debug)]
struct Inner {
    id: TableId,
    name: String,
    columns: HashMap<ColumnId, Arc<ColumnCatalog>>,
    column_id_gen: IntIdGen,
}

pub struct TableCatalog {
    inner: Mutex<Inner>,
}

impl TableCatalog {
    pub fn new(
        id: TableId,
        name: String,
        columns: &[(String, ColumnDesc)],
    ) -> Result<Self, CatalogError> {
        let catalog = TableCatalog {
            inner: Mutex::new(Inner {
                id,
                name,
                columns: HashMap::new(),
                column_id_gen: IntIdGen::new(),
            }),
        };

        if columns.is_empty() {
            return Err(CatalogError::CreateTableWithoutColumn);
        }

        {
            let mut inner = catalog.inner.lock().unwrap();
            for (name, column_desc) in columns {
                let column_id = inner.column_id_gen.next_id();
                inner.columns.insert(
                    column_id,
                    Arc::new(ColumnCatalog::new(
                        column_id,
                        name.to_string(),
                        column_desc.clone(),
                    )),
                );
            }
        }

        Ok(catalog)
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
