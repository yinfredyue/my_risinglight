use super::*;
use crate::catalog::column::ColumnDesc;
use crate::catalog::table::TableCatalog;
use crate::utils::id_gen::IntIdGen;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct Inner {
    id: SchemaId,
    name: String,
    tables: HashMap<TableId, Arc<TableCatalog>>,
    name_to_id: HashMap<String, TableId>,
    id_gen: IntIdGen,
}

pub struct SchemaCatalog {
    inner: Mutex<Inner>,
}

impl SchemaCatalog {
    pub fn new(id: SchemaId, name: String) -> SchemaCatalog {
        SchemaCatalog {
            inner: Mutex::new(Inner {
                id,
                name,
                tables: HashMap::new(),
                name_to_id: HashMap::new(),
                id_gen: IntIdGen::new(),
            }),
        }
    }

    pub fn id(&self) -> SchemaId {
        self.inner.lock().unwrap().id
    }

    pub fn name(&self) -> String {
        self.inner.lock().unwrap().name.clone()
    }

    pub fn add_table(
        &self,
        name: &str,
        columns: &[(String, ColumnDesc)],
    ) -> Result<TableId, CatalogError> {
        let mut inner = self.inner.lock().unwrap();

        if let Some(_) = inner.name_to_id.get(name) {
            return Err(CatalogError::DuplicateName);
        }

        let id = inner.id_gen.next_id();
        let table_catalog = TableCatalog::new(id, name.to_owned(), columns)?;
        inner.tables.insert(id, Arc::new(table_catalog));
        inner.name_to_id.insert(name.to_owned(), id);

        Ok(id)
    }

    pub fn get_table(&self, id: TableId) -> Option<Arc<TableCatalog>> {
        self.inner.lock().unwrap().tables.get(&id).cloned()
    }

    pub fn del_table(&self, id: TableId) {
        let mut inner = self.inner.lock().unwrap();
        let catalog = inner.tables.remove(&id);
        if let Some(catalog) = catalog {
            inner.name_to_id.remove(&catalog.name());
        }
    }
}
