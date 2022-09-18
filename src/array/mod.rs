mod data_chunk;
mod iter;
mod primitive_array;
mod utf8_array;

use super::types::{Value, ValueType};
use iter::ArrayIter;
use primitive_array::{PrimitiveArray, PrimitiveArrayBuilder};
use sqlparser::ast::DataType;
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

    // Get iterator of current array.
    fn iter(&self) -> ArrayIter<'_, Self> {
        ArrayIter::new(self)
    }

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

// ArrayImpl is just a wrapper over different types of Array's
// It might be surprising that ArrayImpl does not implement Array trait.
pub enum ArrayImpl {
    Bool(BoolArray),
    Int32(I32Array),
    Float64(F64Array),
    Utf8(Utf8Array),
}

impl ArrayImpl {
    fn get(&self, idx: usize) -> Value {
        match self {
            ArrayImpl::Bool(a) => match a.get(idx) {
                None => Value::Null,
                Some(v) => Value::Bool(*v),
            },
            ArrayImpl::Int32(a) => match a.get(idx) {
                None => Value::Null,
                Some(v) => Value::Int32(*v),
            },
            ArrayImpl::Float64(a) => match a.get(idx) {
                None => Value::Null,
                Some(v) => Value::Float64(*v),
            },
            ArrayImpl::Utf8(a) => match a.get(idx) {
                None => Value::Null,
                Some(v) => Value::String(String::from(v)),
            },
        }
    }

    fn len(&self) -> usize {
        match self {
            ArrayImpl::Bool(a) => a.len(),
            ArrayImpl::Int32(a) => a.len(),
            ArrayImpl::Float64(a) => a.len(),
            ArrayImpl::Utf8(a) => a.len(),
        }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub type BoolArrayBuilder = PrimitiveArrayBuilder<bool>;
pub type I32ArrayBuilder = PrimitiveArrayBuilder<i32>;
pub type F64ArrayBuilder = PrimitiveArrayBuilder<f64>;

pub enum ArrayBuilderImpl {
    Bool(BoolArrayBuilder),
    Int32(I32ArrayBuilder),
    Float64(F64ArrayBuilder),
    Utf8(Utf8ArrayBuilder),
}

// Surprisingly, ArrayBuilderImpl does not implement ArrayBuilder trait
impl ArrayBuilderImpl {
    pub fn with_capacity(capacity: usize, value_type: ValueType) -> Self {
        match value_type.data_type() {
            DataType::Boolean => Self::Bool(BoolArrayBuilder::with_capacity(capacity)),
            DataType::Int(_) => Self::Int32(I32ArrayBuilder::with_capacity(capacity)),
            DataType::Float(_) | DataType::Double => {
                Self::Float64(F64ArrayBuilder::with_capacity(capacity))
            }
            DataType::Char(_) | DataType::Varchar(_) | DataType::String => {
                Self::Utf8(Utf8ArrayBuilder::with_capacity(capacity))
            }
            _ => panic!("unsupported data type"),
        }
    }

    pub fn push(&mut self, v: &Value) {
        match (self, v) {
            (Self::Bool(a), Value::Bool(v)) => a.push(Some(v)),
            (Self::Int32(a), Value::Int32(v)) => a.push(Some(v)),
            (Self::Float64(a), Value::Float64(v)) => a.push(Some(v)),
            (Self::Utf8(a), Value::String(v)) => a.push(Some(v)),
            (Self::Bool(a), Value::Null) => a.push(None),
            (Self::Int32(a), Value::Null) => a.push(None),
            (Self::Float64(a), Value::Null) => a.push(None),
            (Self::Utf8(a), Value::Null) => a.push(None),
            _ => panic!("failed to push value: type mismatch"),
        }
    }

    pub fn append(&mut self, array_impl: &ArrayImpl) {
        match (self, array_impl) {
            (Self::Bool(builder), ArrayImpl::Bool(arr)) => builder.append(arr),
            (Self::Int32(builder), ArrayImpl::Int32(arr)) => builder.append(arr),
            (Self::Float64(builder), ArrayImpl::Float64(arr)) => builder.append(arr),
            (Self::Utf8(builder), ArrayImpl::Utf8(arr)) => builder.append(arr),
            _ => panic!("failed to append value: type mismatch"),
        }
    }

    pub fn finish(self) -> ArrayImpl {
        match self {
            ArrayBuilderImpl::Bool(b) => ArrayImpl::Bool(b.finish()),
            ArrayBuilderImpl::Int32(b) => ArrayImpl::Int32(b.finish()),
            ArrayBuilderImpl::Float64(b) => ArrayImpl::Float64(b.finish()),
            ArrayBuilderImpl::Utf8(b) => ArrayImpl::Utf8(b.finish()),
        }
    }
}
