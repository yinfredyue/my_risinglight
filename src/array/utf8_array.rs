use std::iter::FromIterator;

use super::{Array, ArrayBuilder};
use bitvec::vec::BitVec;

pub struct Utf8Array {
    is_valid: BitVec, // is_valid.len() = # of items
    data: Vec<u8>,
    offset: Vec<usize>, // offset[i], offset[i+1] is (start, end) of element i
}

impl Array for Utf8Array {
    type Builder = Utf8ArrayBuilder;

    type Item = str;

    fn get(&self, idx: usize) -> Option<&Self::Item> {
        if self.is_valid.len() <= idx || !self.is_valid[idx] {
            None
        } else {
            std::str::from_utf8(&self.data[self.offset[idx]..self.offset[idx + 1]]).ok()
        }
    }

    fn len(&self) -> usize {
        return self.is_valid.len();
    }
}

// Enable `collect()` an array from iterator of `Option<&str>` or `Option<String>`.
// Use AsRef<str> such that we suppport both &str and String!
impl<Str: AsRef<str>> FromIterator<Option<Str>> for Utf8Array {
    fn from_iter<I: IntoIterator<Item = Option<Str>>>(iter: I) -> Self {
        let mut builder = Utf8ArrayBuilder::with_capacity(0);
        for s in iter.into_iter() {
            if let Some(s) = s {
                builder.push(Some(s.as_ref()))
            } else {
                builder.push(None)
            }
        }
        builder.finish()
    }
}

pub struct Utf8ArrayBuilder {
    is_valid: BitVec,
    data: Vec<u8>,
    offset: Vec<usize>,
}

impl ArrayBuilder for Utf8ArrayBuilder {
    type Array = Utf8Array;

    fn with_capacity(capacity: usize) -> Self {
        let mut offset = Vec::with_capacity(capacity + 1);
        offset.push(0);
        Self {
            is_valid: BitVec::with_capacity(capacity),
            data: Vec::with_capacity(capacity),
            offset,
        }
    }

    fn push(&mut self, value: Option<&<Self::Array as Array>::Item>) {
        self.is_valid.push(value.is_some());
        if let Some(v) = value {
            self.data.extend_from_slice(v.as_bytes());
        }
        self.offset.push(self.data.len());
    }

    fn append(&mut self, other: &Self::Array) {
        self.is_valid.extend(&other.is_valid);
        self.data.extend(&other.data);

        let curr_len = self.offset.len();
        let curr_offset = self.offset.last().copied().unwrap();
        self.offset.extend(&other.offset[1..]);
        for i in 0..other.len() {
            self.offset[curr_len + i] += curr_offset;
        }
    }

    fn finish(self) -> Self::Array {
        Utf8Array {
            is_valid: self.is_valid,
            data: self.data,
            offset: self.offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let vals = vec![None, Some("1"), None, Some("3")];

        let mut builder = Utf8ArrayBuilder::with_capacity(0);
        for v in vals {
            builder.push(v);
        }
        let arr = builder.finish();

        assert_eq!(arr.len(), 4);
        assert_eq!(arr.get(0), None);
        assert_eq!(arr.get(1), Some("1"));
        assert_eq!(arr.get(2), None);
        assert_eq!(arr.get(3), Some("3"));
    }

    #[test]
    fn test_collect() {
        let iter = [None, Some("1"), None, Some("3")].into_iter();
        let array = iter.clone().collect::<Utf8Array>();
        assert_eq!(array.iter().collect::<Vec<_>>(), iter.collect::<Vec<_>>());
    }
}
