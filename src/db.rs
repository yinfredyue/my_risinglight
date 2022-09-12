use crate::executor::Executor;
use crate::parser::Parser;

pub use crate::executor::ExecutionError;
pub use sqlparser::parser::ParserError;

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("parsing error: {0}")]
    Parse(#[from] ParserError),

    #[error("execution error: {0}")]
    Execute(#[from] ExecutionError),
}

pub struct Database {}

impl Database {
    pub fn new() -> Self {
        Database {}
    }

    pub fn run(&self, sql: &str) -> Result<Vec<String>, DbError> {
        let parser = Parser::new();
        let executor = Executor::new();

        let stmts = parser.parse_sql(sql)?;
        let results = executor.execute(stmts[0].to_owned())?;

        Ok(results)
    }
}