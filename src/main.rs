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
mod table_scan;

use quicli::prelude::*;
use structopt::StructOpt;

use buffer_manager::BufferManager;

#[derive(Debug, StructOpt)]
struct CliArgs {
    /// How many lines to get
    #[structopt(long = "count", short = "n", default_value = "3")]
    count: usize,

    /// The file to read
    file: String,
    // Quick and easy logging setup you get for free with quicli
    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn main() -> CliResult {
    let mm = BufferManager::new(1000);
    println!("{}", mm);

    let args = CliArgs::from_args();
    args.verbosity.setup_env_logger("head")?;

    let content = read_file(&args.file)?;
    let content_lines = content.lines();
    let first_n_lines = content_lines.take(args.count);

    info!("Reading first {} lines of {:?}", args.count, args.file);

    for line in first_n_lines {
        println!("{}", line);
    }

    Ok(())
}
