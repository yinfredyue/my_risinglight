mod statements;

use crate::binder::statements::BoundCreateTable;
use crate::parser::Statement;

pub enum BoundStatement {
    CreateTable(BoundCreateTable),
}

#[derive(Debug, thiserror::Error)]
pub enum BindingError {}

pub struct Binder {}

impl Binder {
    pub fn new() -> Self {
        todo!()
    }

    pub fn bind(&self, stmt: &Statement) -> Result<BoundStatement, BindingError> {
        todo!()
    }
}
