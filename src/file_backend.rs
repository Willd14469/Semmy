use crate::env::{get_bool, get_bool_with_default};
use crate::parser::parse;
use crate::semantic_version::{SemanticVersion, SemanticVersionBuilder};
use colored::Colorize;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

static LOCAL_FILE_NAME: &str = ".sem_ver";

pub struct FileInterface;

impl FileInterface {
    pub fn load_and_parse() -> anyhow::Result<SemanticVersion> {
        let verbose = get_bool_with_default("VERBOSE", false);
        let coloured = get_bool_with_default("COLOURED", false);

        let file = OpenOptions::new().read(true).open(LOCAL_FILE_NAME);

        if verbose {
            if coloured {
                println!("📖 Reading file {}", LOCAL_FILE_NAME.magenta())
            } else {
                println!("📖 Reading file {}", LOCAL_FILE_NAME)
            }
        }

        if file.is_err() {
            // If the file does not exist then create the default semvar object

            if verbose {
                println!("🔲 Creating {} version...", "new".bright_red())
            }

            return Ok(SemanticVersionBuilder::default().build()?);
        };

        let mut file = file.unwrap();

        let mut raw_string = String::new();
        file.read_to_string(&mut raw_string)?;

        let result = parse(raw_string.leak())?;

        Ok(result)
    }

    pub fn write(version: SemanticVersion) -> anyhow::Result<()> {
        let verbose = get_bool_with_default("VERBOSE", false);
        let coloured = get_bool_with_default("COLOURED", false);

        if verbose {
            if coloured {
                println!("📖 Writing to file {}", LOCAL_FILE_NAME.magenta())
            } else {
                println!("📖 Writing to file {}", LOCAL_FILE_NAME)
            }
        }

        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(LOCAL_FILE_NAME)?;

        file.write_all(version.to_string().as_bytes())?;

        Ok(())
    }
}
