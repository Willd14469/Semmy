mod cli;
mod env;
mod file_backend;
mod parser;
mod semantic_version;

#[macro_use]
extern crate derive_builder;

use crate::cli::Cli;
use clap::Parser;

fn main() {
    let args = Cli::parse();

    args.execute();
}
