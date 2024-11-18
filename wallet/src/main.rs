use anyhow::Result;
use clap::{Command, Parser, Subcommand};
use kanal::bounded;
use tokio::time::{self, Duration};
use std::io::{self, Write};
use std::path::PathBuf;
use btclib::types::Transaction;

mod core;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
    #[arg(short, long, value_name = "ADDRESS")]
    node: Option<String>,
}
#[derive(Subcommand)]
enum Commands {
    GenerateConfig {
        #[arg(short, long, value_name = "FILE")]
        output: PathBuf,
    },
}
fn main() {
    println!("Hello, world!");
}
