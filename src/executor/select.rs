use crate::array::ArrayImpl;
use crate::binder::BoundExpr;

use crate::array::DataChunk;
use crate::binder::BoundSelect;

use super::ExecutionError;
use super::Executor;

pub struct SelectExecutor {
    stmt: BoundSelect,
}

impl Executor for SelectExecutor {
    fn execute(&self) -> Result<DataChunk, ExecutionError> {
        let arrays = self
            .stmt
            .exprs
            .iter()
            .map(|e| match e {
                BoundExpr::Constant(v) => ArrayImpl::from(v),
            })
            .collect();
        Ok(DataChunk::new(arrays))
    }
}

impl SelectExecutor {
    pub fn new(bound_stmt: BoundSelect) -> Self {
        SelectExecutor { stmt: bound_stmt }
    }
}
