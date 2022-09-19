use crate::binder::BoundInsert;

use super::Executor;

pub struct InsertExecutor {}

impl InsertExecutor {
    pub fn new(bound_stmt: BoundInsert) -> Self {
        todo!()
    }
}

impl Executor for InsertExecutor {
    fn execute(&self) -> Result<String, super::ExecutionError> {
        todo!()
    }
}

