mod column;
mod database;
mod schema;
mod table;

type SchemaId = u32;

type TableId = u32;

type ColumnId = u32;

#[derive(Debug, thiserror::Error)]
pub enum CatalogError {
    #[error("duplicata name")]
    DuplicateName,
}


pub use database::DatabaseCatalog;

