mod column;
mod database;
mod schema;
mod table;

pub type SchemaId = u32;

pub type TableId = u32;

pub type ColumnId = u32;

// GlobalTableId = <SchemaId, TableId>
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalTableId {
    pub schema_id: SchemaId,
    pub table_id: TableId,
}

impl GlobalTableId {
    pub fn new(schema_id: SchemaId, table_id: TableId) -> Self {
        GlobalTableId {
            schema_id,
            table_id,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CatalogError {
    #[error("duplicata name")]
    DuplicateName,
    #[error("cannot create table with no column")]
    CreateTableWithoutColumn,
}

pub use column::ColumnDesc;
pub use database::DatabaseCatalog;

pub const DEFAULT_SCHEMA_NAME: &'static str = "postgres";
