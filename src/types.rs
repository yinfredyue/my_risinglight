// 一个值的数据类型，不考虑空值
// 为了方便，我们可以直接使用 sqlparser 中定义的类型
pub use sqlparser::ast::DataType;

pub struct ValueType {
    data_type: DataType,
    nullable: bool,
}

impl ValueType {
    pub const fn new(data_type: DataType, nullable: bool) -> Self {
        ValueType { data_type, nullable }
    }

    pub fn is_nullable(&self) -> bool {
        self.nullable
    }

    pub fn data_type(&self) -> DataType {
        self.data_type.clone()
    }
}

#[derive(Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Int32(i32),
    Float64(f64),
    String(String),
}
