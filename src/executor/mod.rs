use crate::{binder::BoundStatement, catalog::{DatabaseCatalog, CatalogError}};
use std::sync::Arc;

use self::create::CreateTableExecutor;

mod create;

#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    #[error("catalog error: {0}")]
    CatalogError(#[from] CatalogError)
}

pub struct ExecutorBuilder {
    catalog: Arc<DatabaseCatalog>,
}

impl ExecutorBuilder {
    pub fn new(catalog: Arc<DatabaseCatalog>) -> Self {
        ExecutorBuilder { catalog }
    }

    pub fn build(&self, bound_stmt: BoundStatement) -> Box<dyn Executor> {
        let executor = match bound_stmt {
            BoundStatement::CreateTable(bound) => {
                CreateTableExecutor::new(bound, Arc::clone(&self.catalog))
            }
        };
        Box::new(executor)
    }
}

pub trait Executor {
    fn execute(&self) -> Result<String, ExecutionError>;
}

// pub struct Executor {}

// impl Executor {
//     pub fn new() -> Self {
//         Self {}
//     }

//     pub fn execute(&self, stmt: Statement) -> Result<Vec<String>, ExecutionError> {
//         match stmt {
//             Statement::Query(query_stmt) => match *query_stmt.body {
//                 SetExpr::Select(select) => {
//                     let results = select
//                         .projection
//                         .iter()
//                         .map(|item| match item {
//                             SelectItem::UnnamedExpr(expr) => match expr {
//                                 Expr::Value(v) => match v {
//                                     Value::SingleQuotedString(s) => s.clone(),
//                                     Value::Number(s, _) => s.clone(),
//                                     _ => todo!(),
//                                 },
//                                 _ => todo!(),
//                             },
//                             _ => todo!(),
//                         })
//                         .collect();
//                     Ok(results)
//                 }
//                 _ => todo!(),
//             },
//             _ => todo!(),
//         }
//     }
// }
