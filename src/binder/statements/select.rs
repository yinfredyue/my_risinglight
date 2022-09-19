use sqlparser::ast::{Query, SelectItem, SetExpr};

use crate::binder::{expression::BoundExpr, Binder, BindingError};

pub struct BoundSelect {
    pub exprs: Vec<BoundExpr>,
}

impl Binder {
    pub fn bind_select(&self, query: &Query) -> Result<BoundSelect, BindingError> {
        match query.body.as_ref() {
            SetExpr::Select(select) => {
                let exprs = select
                    .projection
                    .iter()
                    .map(|item| match item {
                        SelectItem::UnnamedExpr(expr) => self.bind_expr(expr),
                        _ => todo!(),
                    })
                    .collect();
                Ok(BoundSelect { exprs })
            }
            _ => todo!(),
        }
    }
}
