use std::sync::Arc;

use super::Executor;
use crate::{binder::BoundCreateTable, catalog::DatabaseCatalog};

pub struct CreateTableExecutor {
    stmt: BoundCreateTable,
    catalog: Arc<DatabaseCatalog>,
}

impl CreateTableExecutor {
    pub fn new(stmt: BoundCreateTable, catalog: Arc<DatabaseCatalog>) -> Self {
        CreateTableExecutor { stmt, catalog }
    }
}

impl Executor for CreateTableExecutor {
    fn execute(&self) -> Result<String, super::ExecutionError> {
        let schema = self.catalog.get_schema(self.stmt.schema_id).unwrap();
        let table_id = schema.add_table(&self.stmt.name, &(self.stmt.columns[..]))?;

        Ok(format!(
            "Table {} created. Table id = {}",
            self.stmt.name, table_id
        ))
    }
}
