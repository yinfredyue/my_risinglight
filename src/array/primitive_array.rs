use super::{Array, ArrayBuilder};
use std::fmt::Debug;

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
    // Add new fields here, if any
    data: Vec<T>,
}

impl<T: Primitive> Array for PrimitiveArray<T> {
    type Builder = PrimitiveArrayBuilder<T>;

    type Item = T;

    fn get(&self, idx: usize) -> Option<&Self::Item> {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    // ...
}

/// A builder that constructs a [`PrimitiveArray`] from `Option<T>`.
pub struct PrimitiveArrayBuilder<T: Primitive> {
    // Add new fields here, if any
    data: Vec<T>,
}

impl<T: Primitive> ArrayBuilder for PrimitiveArrayBuilder<T> {
    type Array = PrimitiveArray<T>;

    fn with_capacity(capacity: usize) -> Self {
        todo!()
    }

    fn push(&mut self, value: Option<&<Self::Array as Array>::Item>) {
        todo!()
    }

    fn append(&mut self, other: &Self::Array) {
        todo!()
    }

    fn finish(self) -> Self::Array {
        todo!()
    }
}
