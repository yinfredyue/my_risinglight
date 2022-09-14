use sqlparser::ast::{Expr, SelectItem, SetExpr, Value};

use crate::binder::BoundSelect;

use super::ExecutionError;
use super::Executor;

pub struct SelectExecutor {
    stmt: BoundSelect,
}

impl Executor for SelectExecutor {
    fn execute(&self) -> Result<String, ExecutionError> {
        match self.stmt.query.body.as_ref() {
            SetExpr::Select(select) => Ok(select
                .projection
                .iter()
                .map(|item| match item {
                    SelectItem::UnnamedExpr(expr) => match expr {
                        Expr::Value(v) => match v {
                            Value::SingleQuotedString(s) => s.clone(),
                            Value::Number(s, _) => s.clone(),
                            _ => todo!(),
                        },
                        _ => todo!(),
                    },
                    _ => todo!(),
                })
                .collect::<Vec<String>>()
                .join(", ")
                .to_string()),
            _ => todo!(),
        }
    }
}

impl SelectExecutor {
    pub fn new(bound_stmt: BoundSelect) -> Self {
        SelectExecutor { stmt: bound_stmt }
    }
}
