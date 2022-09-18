use super::{ArrayBuilderImpl, ArrayImpl};
use std::{iter::FromIterator, sync::Arc};

// A datachunk is a collection of arrays.
//
// It's a horizontal subset of a table/query result.
pub struct DataChunk {
    arrays: Arc<[ArrayImpl]>,
}

impl FromIterator<ArrayImpl> for DataChunk {
    fn from_iter<I: IntoIterator<Item = ArrayImpl>>(iter: I) -> Self {
        DataChunk {
            arrays: iter.into_iter().collect(),
        }
    }
}

impl DataChunk {
    pub fn num_rows(&self) -> usize {
        self.arrays[0].len()
    }

    pub fn arrays(&self) -> &[ArrayImpl] {
        &self.arrays
    }
}
