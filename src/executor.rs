#![allow(unused_variables)]

use sqlparser::ast::{Expr, SelectItem, SetExpr, Statement, Value};

pub struct Executor {}

#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {}

impl Executor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&self, stmt: Statement) -> Result<Vec<String>, ExecutionError> {
        match stmt {
            Statement::Query(query_stmt) => match *query_stmt.body {
                SetExpr::Select(select) => {
                    let results = select
                        .projection
                        .iter()
                        .map(|item| match item {
                            SelectItem::UnnamedExpr(expr) => match expr {
                                Expr::Value(v) => match v {
                                    Value::SingleQuotedString(s) => s.clone(),
                                    Value::Number(s, _) => s.clone(),
                                    Value::EscapedStringLiteral(_) => todo!(),
                                    Value::NationalStringLiteral(_) => todo!(),
                                    Value::HexStringLiteral(_) => todo!(),
                                    Value::DoubleQuotedString(_) => todo!(),
                                    Value::Boolean(_) => todo!(),
                                    Value::Interval {
                                        value,
                                        leading_field,
                                        leading_precision,
                                        last_field,
                                        fractional_seconds_precision,
                                    } => todo!(),
                                    Value::Null => todo!(),
                                    Value::Placeholder(_) => todo!(),
                                },
                                _ => todo!(),
                            },
                            SelectItem::ExprWithAlias { expr, alias } => todo!(),
                            SelectItem::QualifiedWildcard(_) => todo!(),
                            SelectItem::Wildcard => todo!(),
                        })
                        .collect();
                    Ok(results)
                }
                SetExpr::Query(_) => todo!(),
                SetExpr::SetOperation {
                    op,
                    all,
                    left,
                    right,
                } => todo!(),
                SetExpr::Values(_) => todo!(),
                SetExpr::Insert(_) => todo!(),
            },
            Statement::Analyze {
                table_name,
                partitions,
                for_columns,
                columns,
                cache_metadata,
                noscan,
                compute_statistics,
            } => todo!(),
            Statement::Truncate {
                table_name,
                partitions,
            } => todo!(),
            Statement::Msck {
                table_name,
                repair,
                partition_action,
            } => todo!(),
            Statement::Insert {
                or,
                into,
                table_name,
                columns,
                overwrite,
                source,
                partitioned,
                after_columns,
                table,
                on,
            } => todo!(),
            Statement::Directory {
                overwrite,
                local,
                path,
                file_format,
                source,
            } => todo!(),
            Statement::Copy {
                table_name,
                columns,
                to,
                target,
                options,
                legacy_options,
                values,
            } => todo!(),
            Statement::Close { cursor } => todo!(),
            Statement::Update {
                table,
                assignments,
                from,
                selection,
            } => todo!(),
            Statement::Delete {
                table_name,
                using,
                selection,
            } => todo!(),
            Statement::CreateView {
                or_replace,
                materialized,
                name,
                columns,
                query,
                with_options,
            } => todo!(),
            Statement::CreateTable {
                or_replace,
                temporary,
                external,
                global,
                if_not_exists,
                name,
                columns,
                constraints,
                hive_distribution,
                hive_formats,
                table_properties,
                with_options,
                file_format,
                location,
                query,
                without_rowid,
                like,
                clone,
                engine,
                default_charset,
                collation,
                on_commit,
                on_cluster,
            } => todo!(),
            Statement::CreateVirtualTable {
                name,
                if_not_exists,
                module_name,
                module_args,
            } => todo!(),
            Statement::CreateIndex {
                name,
                table_name,
                columns,
                unique,
                if_not_exists,
            } => todo!(),
            Statement::AlterTable { name, operation } => todo!(),
            Statement::Drop {
                object_type,
                if_exists,
                names,
                cascade,
                purge,
            } => todo!(),
            Statement::Declare {
                name,
                binary,
                sensitive,
                scroll,
                hold,
                query,
            } => todo!(),
            Statement::Fetch {
                name,
                direction,
                into,
            } => todo!(),
            Statement::Discard { object_type } => todo!(),
            Statement::SetRole {
                local,
                session,
                role_name,
            } => todo!(),
            Statement::SetVariable {
                local,
                hivevar,
                variable,
                value,
            } => todo!(),
            Statement::SetNames {
                charset_name,
                collation_name,
            } => todo!(),
            Statement::SetNamesDefault {} => todo!(),
            Statement::ShowVariable { variable } => todo!(),
            Statement::ShowVariables { filter } => todo!(),
            Statement::ShowCreate { obj_type, obj_name } => todo!(),
            Statement::ShowColumns {
                extended,
                full,
                table_name,
                filter,
            } => todo!(),
            Statement::ShowTables {
                extended,
                full,
                db_name,
                filter,
            } => todo!(),
            Statement::ShowCollation { filter } => todo!(),
            Statement::Use { db_name } => todo!(),
            Statement::StartTransaction { modes } => todo!(),
            Statement::SetTransaction {
                modes,
                snapshot,
                session,
            } => todo!(),
            Statement::Comment {
                object_type,
                object_name,
                comment,
            } => todo!(),
            Statement::Commit { chain } => todo!(),
            Statement::Rollback { chain } => todo!(),
            Statement::CreateSchema {
                schema_name,
                if_not_exists,
            } => todo!(),
            Statement::CreateDatabase {
                db_name,
                if_not_exists,
                location,
                managed_location,
            } => todo!(),
            Statement::CreateFunction {
                temporary,
                name,
                class_name,
                using,
            } => todo!(),
            Statement::Assert { condition, message } => todo!(),
            Statement::Grant {
                privileges,
                objects,
                grantees,
                with_grant_option,
                granted_by,
            } => todo!(),
            Statement::Revoke {
                privileges,
                objects,
                grantees,
                granted_by,
                cascade,
            } => todo!(),
            Statement::Deallocate { name, prepare } => todo!(),
            Statement::Execute { name, parameters } => todo!(),
            Statement::Prepare {
                name,
                data_types,
                statement,
            } => todo!(),
            Statement::Kill { modifier, id } => todo!(),
            Statement::ExplainTable {
                describe_alias,
                table_name,
            } => todo!(),
            Statement::Explain {
                describe_alias,
                analyze,
                verbose,
                statement,
            } => todo!(),
            Statement::Savepoint { name } => todo!(),
            Statement::Merge {
                into,
                table,
                source,
                on,
                clauses,
            } => todo!(),
        }
    }
}
