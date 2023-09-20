use crate::env::set_bool;
use crate::file_backend::FileInterface;
use crate::semantic_version::SemanticVersion;
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::fmt::{Debug, Display};
use std::process::exit;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Include verbose output from the process
    #[arg(short, long, default_value("false"))]
    verbose: bool,

    /// Allow coloured output
    #[arg(short, long, default_value("false"))]
    coloured: bool,
}

type ExitCode = u8;

enum ExitCodes {
    AllGood = 0,
    ParseError = 1,
    FileWriteError = 2,
}

impl Into<i32> for ExitCodes {
    fn into(self) -> i32 {
        self as i32
    }
}

impl Cli {
    /// Execute the corresponding command
    pub fn execute(&self) -> ! {
        if self.verbose {
            set_bool("VERBOSE", true)
        }

        if self.coloured {
            set_bool("COLOURED", true)
        }

        let mut semvar_result = FileInterface::load_and_parse();

        if let Err(error) = semvar_result {
            eprintln!("{}", error);
            exit(ExitCodes::ParseError.into());
        }

        let mut semvar = semvar_result.unwrap();

        match self.command {
            Commands::Major => {
                self.run_major(&mut semvar);
            }
            Commands::Minor => self.run_minor(&mut semvar),
            Commands::Patch => self.run_patch(&mut semvar),
            Commands::Build => self.run_build(&mut semvar),
            Commands::Init => {
                // Let the file interface handle trying to load the file or create the default and
                // then let it get written

                // If should not alter an existing version if it does exist
            }
            Commands::Alpha => self.run_alpha(&mut semvar),
            Commands::Beta => self.run_beta(&mut semvar),
            Commands::Release => self.run_release(&mut semvar),
            Commands::Get => {
                if self.coloured {
                    println!("{}", semvar.coloured_to_string())
                } else {
                    println!("{}", semvar.to_string());
                }

                exit(ExitCodes::AllGood.into());
            }
        };

        let file_write_result = FileInterface::write(semvar);

        if let Err(error) = file_write_result {
            eprintln!("{}", error);
            exit(ExitCodes::FileWriteError.into())
        }

        exit(0);
    }
}

impl Cli {
    fn run_major(&self, semvar: &mut SemanticVersion) {
        semvar.inc_major();
    }

    fn run_minor(&self, semvar: &mut SemanticVersion) {
        semvar.inc_major()
    }

    fn run_patch(&self, semvar: &mut SemanticVersion) {
        semvar.inc_patch()
    }

    fn run_build(&self, semvar: &mut SemanticVersion) {
        semvar.inc_build()
    }

    fn run_alpha(&self, semvar: &mut SemanticVersion) {
        semvar.set_alpha()
    }

    fn run_beta(&self, semvar: &mut SemanticVersion) {
        semvar.set_beta()
    }

    fn run_release(&self, semvar: &mut SemanticVersion) {
        semvar.remove_flag()
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Increment the major version and reset minor, patch and build
    Major,
    /// Increment the Minor version and reset patch and build
    Minor,
    /// Increment the Patch version and reset build
    Patch,
    /// Increment the Build version, does not reset the alpha/beta tags
    Build,

    /// Set the alpha flag
    Alpha,
    /// Set the beta flag
    Beta,
    /// Remove the alpha/beta flags
    Release,

    /// Initialize a default version file starting at 0.1.0
    Init,

    /// Print the version in the current working directory - returns non-zero error code if not initialized
    Get,
}
