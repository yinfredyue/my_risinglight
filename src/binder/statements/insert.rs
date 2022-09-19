use sqlparser::ast::{SetExpr, Statement};
use std::sync::Arc;

use crate::{
    binder::{expression::BoundExpr, split_name, Binder, BindingError},
    catalog::{ColumnCatalog, SchemaId, TableId},
};

pub struct BoundInsert {
    schema_id: SchemaId,
    table_id: TableId,
    values: Vec<Vec<BoundExpr>>,
}

impl Binder {
    pub fn bind_insert(&self, stmt: &Statement) -> Result<BoundInsert, BindingError> {
        match stmt {
            Statement::Insert {
                table_name,
                columns,
                source,
                ..
            } => {
                println!("{:#?}", stmt);

                // lookup schema and table using name
                let (schema_name, table_name) = split_name(table_name)?;

                let schema = self
                    .catalog
                    .get_schema_by_name(schema_name)
                    .ok_or_else(|| BindingError::SchemaNotFound(String::from(schema_name)))?;

                let table = schema
                    .get_table_by_name(table_name)
                    .ok_or_else(|| BindingError::TableNotFound(String::from(table_name)))?;

                // check columns
                let columns: Vec<_> = if columns.is_empty() {
                    table.all_columns()
                } else {
                    columns
                        .iter()
                        .map(|ident| {
                            let col_name = &ident.value;
                            table
                                .get_column_by_name(col_name)
                                .ok_or_else(|| BindingError::TableNotFound(String::from(col_name)))
                        })
                        .collect::<Result<Vec<Arc<ColumnCatalog>>, BindingError>>()?
                };

                let raw_values = match source.body.as_ref() {
                    SetExpr::Values(values) => &values.0,
                    _ => todo!(),
                };

                let mut bound_rows = Vec::new();
                for row in raw_values {
                    if row.len() != columns.len() {
                        return Err(BindingError::TupleLengthMismatch);
                    }

                    let mut exprs = Vec::new();
                    for (idx, expr) in row.iter().enumerate() {
                        let bound_expr = self.bind_expr(expr);
                        let col = &columns[idx];

                        // check null
                        if bound_expr.is_null() && !col.desc().is_nullable() {
                            return Err(BindingError::NonNullableColumn(col.name()));
                        }

                        // check type
                        let expr_type = bound_expr.data_type().unwrap();
                        let col_type = col.desc().value_type().data_type();
                        // TODO: support automatic type cast
                        if expr_type != col_type {
                            return Err(BindingError::TypeMismatch(col_type, expr_type));
                        }

                        exprs.push(bound_expr);
                    }
                    bound_rows.push(exprs);
                }

                Ok(BoundInsert {
                    schema_id: schema.id(),
                    table_id: table.id(),
                    values: bound_rows,
                })
            }
            _ => todo!(),
        }
    }
}
