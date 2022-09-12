use crate::catalog::schema::SchemaCatalog;
use crate::catalog::CatalogError;
use crate::catalog::SchemaId;
use crate::utils::id_gen::IntIdGen;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct Inner {
    schemas: HashMap<SchemaId, Arc<SchemaCatalog>>,
    name_to_id: HashMap<String, SchemaId>,
    id_gen: IntIdGen,
}

// The same Arc<Mutex<T>> pattern. Arc is not be part of the struct, but
// will be added when catalog is actually accessed.
pub struct DatabaseCatalog {
    inner: Mutex<Inner>,
}

impl DatabaseCatalog {
    pub fn new() -> Self {
        DatabaseCatalog {
            inner: Mutex::new(Inner {
                schemas: HashMap::new(),
                name_to_id: HashMap::new(),
                id_gen: IntIdGen::new(),
            }),
        }
    }

    pub fn add_schema(&self, name: &str) -> Result<SchemaId, CatalogError> {
        let mut inner = self.inner.lock().unwrap();

        let schema_id = inner.id_gen.next_id();
        if inner.name_to_id.contains_key(name) {
            return Err(CatalogError::DuplicateName);
        }

        let schema_catalog = SchemaCatalog::new(schema_id, name.to_owned());
        inner.schemas.insert(schema_id, Arc::new(schema_catalog));
        inner.name_to_id.insert(name.to_owned(), schema_id);
        Ok(schema_id)
    }

    pub fn get_schema(&self, id: SchemaId) -> Option<Arc<SchemaCatalog>> {
        self.inner.lock().unwrap().schemas.get(&id).cloned()
    }

    pub fn del_schema(&self, id: SchemaId) {
        let mut inner = self.inner.lock().unwrap();

        let schema_catalog = inner.schemas.remove(&id);
        if let Some(schema_catalog) = schema_catalog {
            inner.name_to_id.remove(&schema_catalog.name());
        }
    }
}
