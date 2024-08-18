use crate::models::{slot::Slot, group::Group};

use std::path::PathBuf;

use clap::Parser;

fn validate_file(file: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(file);

    if !path.exists() {
        return Err(format!("'{}' unknown file", file));
    } else if path.is_dir() {
        return Err(format!("'{}' is a directory", file));
    }

    Ok(path)
}

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long, value_name = "JSON FILE", value_parser = validate_file)]
    groups: PathBuf,

    #[arg(short, long, value_name = "JSON FILE", value_parser = validate_file)]
    slots: PathBuf,

    #[arg(short, long, default_value_t = 21)]
    weeks: usize,
}
