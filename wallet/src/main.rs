use anyhow::Result;
use clap::{Command, Parser, Subcommand};
use kanal::bounded;
use tokio::time::{self, Duration};
use std::io::{self, Write};
use std::path::PathBuf;
use btclib::types::Transaction;
use core::{Config, Core, FeeConfig, FeeType, Recipient};

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

async fn update_utxos(core: Arc<Core>) {
    // ...
}
async fn handle_transactions(
    rx: kanal::AsyncReceiver<Transaction>,
    core: Arc<Core>,
) {
    // ...
}
async fn run_cli(core: Arc<Core>) -> Result<()> {
    // ...
    Ok(())
}
fn generate_dummy_config(path: &PathBuf) -> Result<()> {
    // ...
    Ok(())
}

fn generate_dummy_config(path: &PathBuf) -> Result<()> {
    let dummy_config = Config {
        my_keys: vec![],
        contacts: vec![
            Recipient {
                name: "Alice".to_string(),
                key: PathBuf::from("alice.pub.pem"),
            },
            Recipient {
                name: "Bob".to_string(),
                key: PathBuf::from("bob.pub.pem"),
            },
        ],
        default_node: "127.0.0.1:9000".to_string(),
        fee_config: FeeConfig {
            fee_type: FeeType::Percent,
            value: 0.1,
        },
    };
    let config_str = toml::to_string_pretty(&dummy_config)?;
    std::fs::write(path, config_str)?;
    println!("Dummy config generated at: {}", path.display());
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // ...
    Ok(())
}
