mod cli;
pub mod models;

use clap::Parser;

use cli::Cli;

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}
