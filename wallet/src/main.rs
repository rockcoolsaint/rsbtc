use anyhow::Result;
use clap::{Command, Parser, Subcommand};
use kanal::bounded;
use tokio::time::{self, Duration};
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::Arc;
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
    let mut interval = time::interval(Duration::from_secs(20));
    loop {
        interval.tick().await;
        if let Err(e) = core.fetch_utxos().await {
        eprintln!("Failed to update UTXOs: {}", e);
        }
    }
}
async fn handle_transactions(
    rx: kanal::AsyncReceiver<Transaction>,
    core: Arc<Core>,
) {
    while let Ok(transaction) = rx.recv().await {
        if let Err(e) = core.send_transaction(transaction).await
        {
            eprintln!("Failed to send transaction: {}", e);
        }
    }
}
async fn run_cli(core: Arc<Core>) -> Result<()> {
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let parts: Vec<&str> =
            input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "balance" => { 
                println!(
                    "Current balance: {} satoshis",
                    core.get_balance()
                );
            }
            "send" => {
                if parts.len() != 3 {
                    println!("Usage: send <recipient> <amount>");
                    continue;
                }
                let recipient = parts[1];
                let amount: u64 = parts[2].parse()?;
                let recipient_key = core
                    .config
                    .contacts
                    .iter()
                    .find(|r| r.name == recipient)
                    .ok_or_else(|| {
                        anyhow::anyhow!("Recipient not found")
                    })?
                    .load()?
                    .key;
                if let Err(e) = core.fetch_utxos().await {
                    println!("failed to fetch utxos: {e}");
                };
                let transaction = core
                    .create_transaction(&recipient_key, amount)
                    .await?;
                core.tx_sender.send(transaction).await?;
                println!("Transaction sent successfully");
                core.fetch_utxos().await?;
            }
            "exit" => break,
            _ => println!("Unknown command"),
        }
    }
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
