use sqlparser::ast::{DataType, Expr, Value as ParsedValue};

use crate::types::Value;

use super::Binder;

pub enum BoundExpr {
    Constant(Value),
}

impl BoundExpr {
    pub fn data_type(&self) -> Option<DataType> {
        match self {
            BoundExpr::Constant(v) => v.data_type(),
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            BoundExpr::Constant(v) => {
                if let Value::Null = v {
                    return true;
                }
                return false;
            }
        }
    }
}

impl Binder {
    pub fn bind_expr(&self, expr: &Expr) -> BoundExpr {
        match expr {
            Expr::Value(value) => match value {
                ParsedValue::Null => BoundExpr::Constant(Value::Null),
                ParsedValue::Boolean(b) => BoundExpr::Constant(Value::Bool(b.clone())),
                ParsedValue::Number(str, _) => {
                    if let Ok(i) = str.parse::<i32>() {
                        BoundExpr::Constant(Value::Int32(i))
                    } else if let Ok(f) = str.parse::<f64>() {
                        BoundExpr::Constant(Value::Float64(f))
                    } else {
                        panic!("invalid numeric literal: {}", str);
                    }
                }
                ParsedValue::SingleQuotedString(s) | ParsedValue::DoubleQuotedString(s) => {
                    BoundExpr::Constant(Value::String(s.clone()))
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}
