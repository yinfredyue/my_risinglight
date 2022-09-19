use crate::{
    binder::BoundStatement,
    catalog::{CatalogError, DatabaseCatalog},
};
use std::sync::Arc;

use self::{create::CreateTableExecutor, insert::InsertExecutor, select::SelectExecutor};

mod create;
mod insert;
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
            BoundStatement::Select(bound) => Box::new(SelectExecutor::new(bound)),
            BoundStatement::Insert(bound) => Box::new(InsertExecutor::new(bound)),
            BoundStatement::CreateTable(bound) => {
                Box::new(CreateTableExecutor::new(bound, Arc::clone(&self.catalog)))
            }
        }
    }
}

pub trait Executor {
    fn execute(&self) -> Result<String, ExecutionError>;
}
