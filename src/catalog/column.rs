use crate::{catalog::ColumnId, types::ValueType};
use sqlparser::{ast::ColumnDef, ast::ColumnOption};

#[derive(Debug, Clone)]
pub struct ColumnDesc {
    name: String,
    is_primary: bool,
    value_type: ValueType,
}

impl ColumnDesc {
    pub fn is_nullable(&self) -> bool {
        self.value_type.is_nullable()
    }
    pub fn is_primary(&self) -> bool {
        self.is_primary
    }
    pub fn value_type(&self) -> ValueType {
        self.value_type.clone()
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl From<ColumnDef> for ColumnDesc {
    fn from(def: ColumnDef) -> Self {
        let mut primary = false;
        let mut is_nullable = true;
        for option_def in def.options {
            match option_def.option {
                ColumnOption::NotNull => {
                    is_nullable = false;
                }
                ColumnOption::Unique { is_primary } => {
                    primary = is_primary;
                    is_nullable = false;
                }
                _ => (),
            };
        }

        ColumnDesc {
            name: def.name.value,
            is_primary: primary,
            value_type: ValueType::new(def.data_type, is_nullable),
        }
    }
}

#[derive(Debug)]
pub struct ColumnCatalog {
    id: ColumnId,
    name: String,
    desc: ColumnDesc,
}

impl ColumnCatalog {
    pub fn new(id: ColumnId, name: String, desc: ColumnDesc) -> Self {
        ColumnCatalog { id, name, desc }
    }
    pub fn id(&self) -> ColumnId {
        self.id
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn desc(&self) -> ColumnDesc {
        self.desc.clone()
    }
}
