use sqlparser::ast::Query;

use crate::binder::{Binder, BindingError};

pub struct BoundSelect {
    pub query: Query,
}

impl Binder {
    pub fn bind_select(&self, query: &Query) -> Result<BoundSelect, BindingError> {
        Ok(BoundSelect {
            query: query.clone(),
        })
    }
}
