use super::{Array, ArrayBuilder};
use bitvec::vec::BitVec;
use std::{fmt::Debug, iter::FromIterator};

/// A collection of primitive types, such as `i32`, `f32`.
pub trait Primitive:
    PartialOrd + PartialEq + Debug + Copy + Send + Sync + Sized + Default + 'static
{
}

impl Primitive for bool {}
impl Primitive for i32 {}
impl Primitive for f64 {}

#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveArray<T: Primitive> {
    // is_valid.len() = # of items
    is_valid: BitVec,
    data: Vec<T>,
}

impl<T: Primitive> Array for PrimitiveArray<T> {
    type Builder = PrimitiveArrayBuilder<T>;

    type Item = T;

    fn get(&self, idx: usize) -> Option<&Self::Item> {
        if self.is_valid.len() <= idx || !self.is_valid[idx] {
            None
        } else {
            Some(&self.data[idx])
        }
    }

    fn len(&self) -> usize {
        self.is_valid.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// Enable `collect` from iterator of `Option<T>`
impl<T: Primitive> FromIterator<Option<T>> for PrimitiveArray<T> {
    fn from_iter<I: IntoIterator<Item = Option<T>>>(iter: I) -> Self {
        todo!()
    }
}

// Enable `collect` from iterator of `T`
impl<T: Primitive> FromIterator<T> for PrimitiveArray<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        todo!()
    }
}

/// A builder that constructs a [`PrimitiveArray`] from `Option<T>`.
pub struct PrimitiveArrayBuilder<T: Primitive> {
    is_valid: BitVec,
    data: Vec<T>,
}

impl<T: Primitive> ArrayBuilder for PrimitiveArrayBuilder<T> {
    type Array = PrimitiveArray<T>;

    fn with_capacity(capacity: usize) -> Self {
        Self {
            is_valid: BitVec::with_capacity(capacity),
            data: Vec::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: Option<&<Self::Array as Array>::Item>) {
        self.is_valid.push(value.is_some());
        self.data.push(value.cloned().unwrap_or_default());
    }

    fn append(&mut self, other: &Self::Array) {
        self.is_valid.extend(&other.is_valid);
        self.data.extend(&other.data);
    }

    fn finish(self) -> Self::Array {
        Self::Array {
            is_valid: self.is_valid,
            data: self.data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let num_to_val = |x| if x % 2 == 0 { Some(x) } else { None };

        let mut builder = PrimitiveArrayBuilder::with_capacity(0);
        for v in 0..1000 {
            builder.push(num_to_val(v).as_ref());
        }
        let arr = builder.finish();

        assert_eq!(arr.len(), 1000);
        for i in 0..1000 {
            assert_eq!(num_to_val(i), arr.get(i as usize).cloned());
        }
    }

    #[test]
    fn test_append() {
        let num_to_val = |x| if x % 2 == 0 { Some(x) } else { None };

        // 0..499
        let mut builder1 = PrimitiveArrayBuilder::with_capacity(500);
        for v in 0..500 {
            builder1.push(num_to_val(v).as_ref());
        }
        let arr1 = builder1.finish();

        // 0..499 + 500..999
        let mut builder2 = PrimitiveArrayBuilder::with_capacity(0);
        builder2.append(&arr1);
        for v in 500..1000 {
            builder2.push(num_to_val(v).as_ref());
        }
        let arr2 = builder2.finish();

        assert_eq!(arr2.len(), 1000);
        for i in 0..1000 {
            assert_eq!(num_to_val(i), arr2.get(i as usize).cloned());
        }
    }
}
