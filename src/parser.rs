pub use sqlparser::ast::Statement;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser as SQLParser;
use sqlparser::parser::ParserError;

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse_sql(&self, sql: &str) -> Result<Vec<Statement>, ParserError> {
        let dialect = PostgreSqlDialect {};
        SQLParser::parse_sql(&dialect, sql)
    }
}
