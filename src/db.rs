use std::sync::Arc;

use crate::binder::Binder;
use crate::catalog::DatabaseCatalog;
use crate::executor::ExecutorBuilder;
use crate::parser::Parser;

pub use crate::binder::BindingError;
pub use crate::catalog::CatalogError;
pub use crate::executor::ExecutionError;
pub use sqlparser::parser::ParserError;

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("parsing error: {0}")]
    Parse(#[from] ParserError),

    #[error("execution error: {0}")]
    Execute(#[from] ExecutionError),

    #[error("catalog error: {0}")]
    Catalog(#[from] CatalogError),

    #[error("binding error: {0}")]
    Bind(#[from] BindingError),
}

struct Inner {
    parser: Parser,
    binder: Binder,
    executor_builder: ExecutorBuilder,
}

pub struct Database {
    inner: Inner,
}

impl Database {
    pub fn new() -> Self {
        let catalog = Arc::new(DatabaseCatalog::new());
        Database {
            inner: Inner {
                parser: Parser::new(),
                binder: Binder::new(Arc::clone(&catalog)),
                executor_builder: ExecutorBuilder::new(Arc::clone(&catalog)),
            },
        }
    }

    pub fn run(&self, sql: &str) -> Result<Vec<String>, DbError> {
        let stmt = self.inner.parser.parse_sql(sql)?.remove(0);
        let bound_stmt = self.inner.binder.bind(&stmt)?;
        let executor = self.inner.executor_builder.build(bound_stmt);
        let results = executor.execute()?;

        Ok(vec![results])
    }
}
