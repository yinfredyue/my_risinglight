use super::ArrayImpl;
use std::{iter::FromIterator, sync::Arc};

// A datachunk is a collection of arrays.
//
// It's a horizontal subset of a table/query result.
#[derive(Clone)]
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
    pub fn new(arrays: Vec<ArrayImpl>) -> Self {
        Self {
            arrays: arrays.into(),
        }
    }

    pub fn num_rows(&self) -> usize {
        if self.arrays.is_empty() {
            0
        } else {
            self.arrays[0].len()
        }
    }

    pub fn arrays(&self) -> &[ArrayImpl] {
        &self.arrays
    }
}

/// Print the chunk as a pretty table.
impl std::fmt::Display for DataChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use prettytable::{format, Table};
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        for i in 0..self.num_rows() {
            let row = self.arrays.iter().map(|a| a.get(i).to_string()).collect();
            table.add_row(row);
        }
        write!(f, "{}", table)
    }
}
