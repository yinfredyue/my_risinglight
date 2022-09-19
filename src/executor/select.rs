use core::panic;

use crate::array::ArrayImpl;
use crate::types::Value;
use sqlparser::ast::{Expr, SelectItem, SetExpr, Value as ParsedValue};

use crate::array::DataChunk;
use crate::binder::BoundSelect;

use super::ExecutionError;
use super::Executor;

pub struct SelectExecutor {
    stmt: BoundSelect,
}

impl Executor for SelectExecutor {
    fn execute(&self) -> Result<DataChunk, ExecutionError> {
        match self.stmt.query.body.as_ref() {
            SetExpr::Select(select) => {
                let item_to_value = |item: &SelectItem| match item {
                    SelectItem::UnnamedExpr(expr) => match expr {
                        Expr::Value(v) => match v {
                            ParsedValue::SingleQuotedString(s) => Value::String(s.clone()),
                            ParsedValue::Boolean(b) => Value::Bool(b.clone()),
                            ParsedValue::Number(s, _) => {
                                if let Ok(i) = s.parse::<i32>() {
                                    Value::Int32(i)
                                } else if let Ok(f) = s.parse::<f64>() {
                                    Value::Float64(f)
                                } else {
                                    panic!("?")
                                }
                            }
                            _ => todo!(),
                        },
                        _ => todo!(),
                    },
                    _ => todo!(),
                };

                let arrays: Vec<ArrayImpl> = select
                    .projection
                    .iter()
                    .map(item_to_value)
                    .map(|x| ArrayImpl::from(&x))
                    .collect();
                Ok(DataChunk::new(arrays))
            }
            _ => todo!(),
        }
    }
}

impl SelectExecutor {
    pub fn new(bound_stmt: BoundSelect) -> Self {
        SelectExecutor { stmt: bound_stmt }
    }
}
