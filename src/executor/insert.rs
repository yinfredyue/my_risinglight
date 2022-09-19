use crate::{binder::BoundInsert, array::DataChunk};

use super::Executor;

pub struct InsertExecutor {
    stmt: BoundInsert,
}

impl InsertExecutor {
    pub fn new(bound_stmt: BoundInsert) -> Self {
        Self { stmt: bound_stmt }
    }
}

impl Executor for InsertExecutor {
    fn execute(&self) -> Result<DataChunk, super::ExecutionError> {
        Ok(DataChunk::new(vec![]))
    }
}

