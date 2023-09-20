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
    // let mut version = SemanticVersionBuilder::default()
    //     .major(1)
    //     .build_iteration(12)
    //     // .as_alpha()
    //     .as_beta()
    //     .build()
    //     .unwrap();
    //
    // version.inc_major();
    // eprintln!("version.to_string() = {:?}", version.to_string());
    //
    // FileInterface::write(version).expect("unable to write to file");
    // let mut version = FileInterface::load_and_parse().unwrap();
    //
    // eprintln!("version = {:?}", version);
    //
    // version.inc_minor();
    //
    // eprintln!("version = {:?}", version);

    let args = Cli::parse();

    args.execute();
}
