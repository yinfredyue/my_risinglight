use crate::{
    binder::{split_name, Binder, BindingError},
    catalog::SchemaId,
};
use sqlparser::ast::Statement;

use crate::catalog::ColumnDesc;

#[derive(Debug)]
pub struct BoundCreateTable {
    pub stmt: Statement,
    pub schema_id: SchemaId,
    pub name: String,
    pub columns: Vec<(String, ColumnDesc)>,
}

impl Binder {
    pub fn bind_create_table(&self, stmt: &Statement) -> Result<BoundCreateTable, BindingError> {
        match stmt {
            Statement::CreateTable { name, columns, .. } => {
                let (schema_name, table_name) = split_name(name)?;
                let schema = self
                    .catalog
                    .get_schema_by_name(schema_name)
                    .ok_or_else(|| BindingError::SchemaNotFound(schema_name.to_string()))?;
                let columns = columns
                    .into_iter()
                    .map(|def| (def.name.to_string(), def.to_owned().into()))
                    .collect();
                let bound = BoundCreateTable {
                    stmt: stmt.clone(),
                    schema_id: schema.id(),
                    name: table_name.to_string(),
                    columns,
                };
                Ok(bound)
            }
            _ => todo!(),
        }
    }
}
