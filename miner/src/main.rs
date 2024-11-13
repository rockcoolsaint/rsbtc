use anyhow::{anyhow, Result};
use std::{env, process::exit};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};
use clap::Parser;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;

use btclib::{crypto::PublicKey, network::Message, types::Block, util::Saveable};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    address: String,
    #[arg(short, long)]
    public_key_file: String,
}

struct Miner;
impl Miner {
    async fn new(
        address: String,
        Public_key: PublicKey
    ) -> Result<Self> {

    }
    async fn run(&self) -> Result<()> {
        // ...
    }
    fn spawn_mining_thread(&self) -> thread::JoinHandle<()> {
        // ...
    }
    async fn fetch_and_validate_template(&self) -> Result<()> {
        // ...
    }
    async fn fetch_template(&self) -> Result<()> {
        // ...
    }
    async fn validate_template(&self) -> Result<()> {
        // ...
    }
    async fn submit_block(&self, block: Block) -> Result<()> {
        // ...
    }
}

#[tokio::main]
async fn main() {
    // parse block path and steps count from the
    // first and second argument respectively
    // let (path, steps) = if let (Some(arg), Some(arg2)) =
    //     (env::args().nth(1), env::args().nth(2))
    // {
    //     (arg, arg2)
    // } else {
    //     eprintln!("Usage: miner <block_file> <steps>");
    //     exit(1);
    // };
    // // parse steps count
    // let steps: usize = if let Ok(s @ 1..=usize::MAX) =
    //     steps.parse()
    // {
    //     s
    // } else {
    //     eprintln!("<steps> should be a positive integer");
    //     exit(1);
    // };
    // // load block from file
    // let og_block = Block::load_from_file(path)
    //     .expect("Failed to load block");
    // let mut block = og_block.clone();

    // while !block.header.mine(steps) {
    //     println!("mining...");
    // }

    // // print mined block and its hash
    // println!("final: {:#?}", block);
    // println!("hash: {}", block.header.hash());

    // fn usage() -> ! {
    //     eprintln!(
    //         "Usage: {} <address> <public_key_file",
    //         env::args().next().unwrap()
    //     );
    //     exit(1);
    // }

    // let address = match  env::args().nth(1) {
    //     Some(address) => address,
    //     None => usage(),
    // };
    // let public_key_file = match env::args().nth(2) {
    //     Some(public_key_file) => public_key_file,
    //     None => usage(),
    // };
    // let Ok(public_key) =
    //     PublicKey::load_from_file(&public_key_file)
    // else {
    //     eprintln!(
    //         "Error reading public key from file {}",
    //         public_key_file
    //     );
    //     exit(1);
    // };
    // println!(
    //     "Connecting to {address} to mine with {public_key:?}",
    // );

    // let mut stream = match TcpStream::connect(&address).await {
    //     Ok(stream) => stream,
    //     Err(e) => {
    //         eprintln!("Failed to connect to server: {}", e);
    //         exit(1);
    //     }

    // };
    // // Ask the node for work
    // println!("request work from {address}");
    // let message = Message::FetchTemplate(public_key);
    // message.send(&mut stream);
    let cli = Cli::parse();
    let public_key =
        PublicKey::load_from_file(&cli.public_key_file)
            .map_err(|e| {
                anyhow!("Error reading public key: {}", e)
            })?;
    let miner = Miner::new(cli.address, public_key).await?;
    miner.run().await
}
