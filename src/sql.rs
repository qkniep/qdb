// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use sqlparser::ast::Statement;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

/*enum Statement {
    Select,
    Insert,
    Update,
    Delete,
    CreateTable(String),
    DropTable(String),
}*/

pub fn parse_sql_statement(sql: &str) -> Result<Vec<Statement>, String> {
    let dialect = GenericDialect {};
    match Parser::parse_sql(&dialect, sql) {
        Ok(ast) => Ok(ast),
        Err(err) => Err(format!("Parsing failed: {}", err)),
    }
    /*let command = sql.split(' ').next();
    if command == None {
        return Err("Empty statement".to_owned());
    }

    match command.unwrap() {
        "select" => parse_select_statement(sql),
        "insert" => parse_insert_statement(sql),
        "update" => Ok(Statement::Update),
        "delete" => Ok(Statement::Delete),
        "create" => {
            let table_name = sql.get(13..).unwrap();
            Ok(Statement::CreateTable(table_name.to_owned()))
        }
        "drop" => {
            let table_name = sql.get(11..).unwrap();
            Ok(Statement::DropTable(table_name.to_owned()))
        }
        cmd => Err(format!("Unknown SQL command: {}", cmd)),
    }*/
}

/*fn parse_select_statement(sql: &str) -> Result<Statement, String> {
    let parts: Vec<&str> = sql.split(' ').collect();
    return Err(format!("Unknown SQL command"));
}

fn parse_insert_statement(sql: &str) -> Result<Statement, String> {
    let parts: Vec<&str> = sql.split(' ').collect();
    let second_word = &(parts[1].to_lowercase());
    if second_word != "into" {
        Err("Syntax error: 'insert' keyword needs to be followed by 'into'".to_owned())
    } else {
        Ok(Statement::Insert)
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select() {
        assert!(parse_sql_statement("select a + b * c").is_ok());
        assert!(parse_sql_statement("select * from table1").is_ok());
        assert!(parse_sql_statement("select id, name, salary from employees").is_ok());
    }

    #[test]
    fn select_from_where() {
        assert!(parse_sql_statement("select id from employees where salary > 100000").is_ok());
        assert!(parse_sql_statement(
            "select id from employees where salary > 100000 and role != \"CEO\""
        )
        .is_ok());
    }
}
