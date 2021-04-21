// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

mod btree;
mod buffer_manager;
mod disk_manager;
mod extensible_hash;
mod external_sort;
mod nested_loop_join;
mod page;
mod relation;
mod replacer;
mod sql;
mod table_scan;

use std::io::{self, Write};

use quicli::prelude::*;
use sqlparser::ast::Statement;
use structopt::StructOpt;

use sql::parse_sql_statement;

#[derive(Debug, StructOpt)]
struct CliArgs {
    /// How many lines to get
    #[structopt(long = "count", short = "n", default_value = "3")]
    count: usize,

    /// The file to read
    file: String,

    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn main() -> CliResult {
    let args = CliArgs::from_args();
    args.verbosity.setup_env_logger("qdb")?;

    print_intro();

    loop {
        let mut buf = String::new();

        print_prompt();
        read_user_input(&mut buf)?;

        if buf.get(0..1) == Some("\\") {
            perform_meta_command(&buf[1..]);
            continue;
        }

        match prepare_statement(&buf) {
            Ok(statement) => execute_statement(statement),
            Err(err) => println!("{}", err),
        }
    }
}

fn perform_meta_command(cmd: &str) {
    if cmd == "?" || cmd == "help" {
        print_help();
    } else if cmd == "q" || cmd == "quit" {
        std::process::exit(0);
    } else {
        println!("Unrecognized command: {}", cmd);
    }
}

fn prepare_statement(sql: &str) -> Result<Vec<Statement>, String> {
    return parse_sql_statement(sql);
}

fn execute_statement(statements: Vec<Statement>) {
    for statement in statements {
        println!("Executing: {:?}", statement);
    }
}

fn print_intro() {
    println!("Welcome to QDB, you can use '\\?' to see available commands or start typing SQL.");
}

fn print_help() {
    println!("\\?, \\help\tShow this overview");
    println!("\\q, \\quit\tClose the current prompt and disconnect");
}

fn print_prompt() {
    print!("qdb > ");
    io::stdout().flush().unwrap();
}

fn read_user_input(buf: &mut String) -> io::Result<()> {
    let n = io::stdin().read_line(buf)?;
    if n <= 0 {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "no input"));
    }

    // Remove trailing newline character
    buf.pop();
    Ok(())
}
