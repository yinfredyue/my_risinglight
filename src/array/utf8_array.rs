use super::{Array, ArrayBuilder};

pub struct Utf8Array {}

impl Array for Utf8Array {
    type Builder = Utf8ArrayBuilder;

    type Item = str;

    fn get(&self, idx: usize) -> Option<&Self::Item> {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }
}

pub struct Utf8ArrayBuilder {}

impl ArrayBuilder for Utf8ArrayBuilder {
    type Array = Utf8Array;

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
