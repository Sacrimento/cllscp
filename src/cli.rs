use crate::models::{group::Group, slot::Slot};
use csv;

use clap::Parser;
use serde::de::DeserializeOwned;

fn parse_csv<T: DeserializeOwned>(fpath: &str) -> Result<Vec<T>, csv::Error> {
    Ok(csv::Reader::from_path(fpath)?
        .deserialize()
        .collect::<Result<Vec<T>, _>>()?)
}

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long, value_name = "CSV FILE", value_parser = parse_csv::<Group>)]
    groups: std::vec::Vec<Group>,

    #[arg(short, long, value_name = "CSV FILE", value_parser = parse_csv::<Slot>)]
    slots: std::vec::Vec<Slot>,

    #[arg(short, long, default_value_t = 21)]
    weeks: usize,
}
