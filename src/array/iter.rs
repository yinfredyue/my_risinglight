use super::Array;

pub struct ArrayIter<'a, A: Array> {
    array: &'a A,
}

impl<'a, A: Array> ArrayIter<'a, A> {
    pub fn new(arr: &A) -> Self {
        todo!()
    }
}

impl<'a, A: Array> Iterator for ArrayIter<'a, A> {
    // What's the type of Item?
    // We know this involves A::Item.
    // Because next() does not consume Array, we must return reference: & A::Item;
    // It must be constrained by lifetime: &'a A::Item;
    // The item can be null, so Option<&'a A::Item>
    type Item = Option<&'a A::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
