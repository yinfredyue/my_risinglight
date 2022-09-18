mod primitive_array;
mod utf8_array;

use primitive_array::{PrimitiveArray, PrimitiveArrayBuilder};
use utf8_array::{Utf8Array, Utf8ArrayBuilder};

/// A trait over all array.
///
/// [`Array`] must be built with an [`ArrayBuilder`]. The array trait provides several
/// unified interface on an array, like `len`, `get` and `iter`.
///
/// The `Builder` associated type is the builder for this array.
/// The `Item` is the item you could retrieve from this array.
///
/// For example, [`PrimitiveArray`] could return an `Option<&u32>`, and [`Utf8Array`] will
/// return an `Option<&str>`.
pub trait Array: Sized + Send + Sync + 'static {
    /// Corresponding builder of this array.
    type Builder: ArrayBuilder<Array = Self>;

    /// Type of element in the array.
    type Item: ToOwned + ?Sized;

    /// Retrieve a reference to value.
    fn get(&self, idx: usize) -> Option<&Self::Item>;

    /// Number of items of array.
    fn len(&self) -> usize;

    /// Get iterator of current array.
    // fn iter(&self) -> ArrayIter<'_, Self> {
    //     ArrayIter::new(self)
    // }

    /// Check if the array has a length of 0.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A trait over all array builders.
///
/// [`ArrayBuilder`] is a trait over all builders. You could build an array with
/// `push` with the help of [`ArrayBuilder`] trait. The `push` function always
/// accepts reference to an element. e.g. for [`PrimitiveArray`],
/// you must do `builder.push(Some(&1))`. For [`Utf8Array`], you must do
/// `builder.push(Some("xxx"))`. Note that you don't need to construct a `String`.
///
/// The associated type `Array` is the type of the corresponding array. It is the
/// return type of `finish`.
pub trait ArrayBuilder: Send + Sync + 'static {
    /// Corresponding `Array` of this builder
    type Array: Array<Builder = Self>;

    /// Create a new builder with `capacity`.
    fn with_capacity(capacity: usize) -> Self;

    /// Append a value to builder.
    fn push(&mut self, value: Option<&<Self::Array as Array>::Item>);

    /// Append an array to builder.
    fn append(&mut self, other: &Self::Array);

    /// Finish build and return a new array.
    fn finish(self) -> Self::Array;
}

pub type BoolArray = PrimitiveArray<bool>;
pub type I32Array = PrimitiveArray<i32>;
pub type F64Array = PrimitiveArray<f64>;

pub enum ArrayImpl {
    Bool(BoolArray),
    Int32(I32Array),
    Float64(F64Array),
    Utf8(Utf8Array),
}

pub type BoolArrayBuilder = PrimitiveArrayBuilder<bool>;
pub type I32ArrayBuilder = PrimitiveArrayBuilder<i32>;
pub type F64ArrayBuilder = PrimitiveArrayBuilder<f64>;

pub enum ArrayBuilderImpl {
    Bool(BoolArrayBuilder),
    Int32(I32ArrayBuilder),
    F64(F64ArrayBuilder),
    Utf8(Utf8ArrayBuilder),
}
