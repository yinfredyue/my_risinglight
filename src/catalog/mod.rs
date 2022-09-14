mod column;
mod database;
mod schema;
mod table;

pub type SchemaId = u32;

pub type TableId = u32;

pub type ColumnId = u32;

#[derive(Debug, thiserror::Error)]
pub enum CatalogError {
    #[error("duplicata name")]
    DuplicateName,
}


pub use database::DatabaseCatalog;
pub use column::ColumnDesc;

pub const DEFAULT_SCHEMA_NAME: &'static str = "default_schema";