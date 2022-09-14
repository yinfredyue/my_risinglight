use crate::{
    binder::BoundStatement,
    catalog::{CatalogError, DatabaseCatalog},
};
use std::sync::Arc;

use self::{create::CreateTableExecutor, select::SelectExecutor};

mod create;
mod select;

#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    #[error("catalog error: {0}")]
    CatalogError(#[from] CatalogError),
}

pub struct ExecutorBuilder {
    catalog: Arc<DatabaseCatalog>,
}

impl ExecutorBuilder {
    pub fn new(catalog: Arc<DatabaseCatalog>) -> Self {
        ExecutorBuilder { catalog }
    }

    pub fn build(&self, bound_stmt: BoundStatement) -> Box<dyn Executor> {
        match bound_stmt {
            BoundStatement::CreateTable(bound) => {
                Box::new(CreateTableExecutor::new(bound, Arc::clone(&self.catalog)))
            }
            BoundStatement::Select(bound) => Box::new(SelectExecutor::new(bound)),
        }
    }
}

pub trait Executor {
    fn execute(&self) -> Result<String, ExecutionError>;
}
