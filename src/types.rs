// 一个值的数据类型，不考虑空值
// 为了方便，我们可以直接使用 sqlparser 中定义的类型
pub use sqlparser::ast::DataType;

#[derive(Debug, Clone)]
pub struct ValueType {
    data_type: DataType,
    nullable: bool,
}

impl ValueType {
    pub const fn new(data_type: DataType, nullable: bool) -> Self {
        ValueType {
            data_type,
            nullable,
        }
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

impl Value {
    pub fn data_type(&self) -> Option<DataType> {
        match self {
            Value::Null => None,
            Value::Bool(_) => Some(DataType::Boolean),
            Value::Int32(_) => Some(DataType::Integer(None)),
            Value::Float64(_) => Some(DataType::Float(None)),
            Value::String(_) => Some(DataType::String),
        }
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Self::Null => String::from("NULL"),
            Self::Bool(v) => v.to_string(),
            Self::Int32(v) => v.to_string(),
            Self::Float64(v) => v.to_string(),
            Self::String(v) => v.to_string(),
        }
    }
}