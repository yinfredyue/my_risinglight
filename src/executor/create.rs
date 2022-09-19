use std::sync::Arc;

use super::Executor;
use crate::{array::DataChunk, binder::BoundCreateTable, catalog::DatabaseCatalog};

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
    fn execute(&self) -> Result<DataChunk, super::ExecutionError> {
        let schema = self.catalog.get_schema(self.stmt.schema_id).unwrap();
        schema.add_table(&self.stmt.name, &(self.stmt.columns[..]))?;
        Ok(DataChunk::new(vec![]))
    }
}
