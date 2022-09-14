mod statements;

use std::sync::Arc;

use super::catalog::DEFAULT_SCHEMA_NAME;
pub use crate::binder::statements::{BoundCreateTable, BoundSelect};
use crate::{catalog::DatabaseCatalog, parser::Statement};
use sqlparser::ast::{Ident, ObjectName};

pub enum BoundStatement {
    Select(BoundSelect),
    CreateTable(BoundCreateTable),
}

#[derive(Debug, thiserror::Error)]
pub enum BindingError {
    #[error("invalid table name: {0:?}")]
    InvalidTableName(Vec<Ident>),
    #[error("schema not found: {0}")]
    SchemaNotFound(String),
    #[error("cannot create table with zero column")]
    EmptyColumns,
}

pub struct Binder {
    catalog: Arc<DatabaseCatalog>,
}

impl Binder {
    pub fn new(catalog: Arc<DatabaseCatalog>) -> Self {
        Binder { catalog }
    }

    pub fn bind(&self, stmt: &Statement) -> Result<BoundStatement, BindingError> {
        match stmt {
            Statement::Query(query) => Ok(BoundStatement::Select(self.bind_select(query)?)),
            Statement::CreateTable { .. } => {
                Ok(BoundStatement::CreateTable(self.bind_create_table(stmt)?))
            }
            _ => todo!(),
        }
    }
}

/// Split an [ObjectName] into `(schema name, table name)`.
fn split_name(name: &ObjectName) -> Result<(&str, &str), BindingError> {
    Ok(match name.0.as_slice() {
        [table] => (DEFAULT_SCHEMA_NAME, &table.value),
        [schema, table] => (&schema.value, &table.value),
        _ => return Err(BindingError::InvalidTableName(name.0.clone())),
    })
}
