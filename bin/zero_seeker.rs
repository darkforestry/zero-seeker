use clap::Arg;
use ethers::types::{H160, U256};
use libsecp256k1::{PublicKey, SecretKey};
use rayon::prelude::*;
use rlp::RlpStream;
use sha3::{Digest, Keccak256, Sha3_256};
use std::error::Error;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tiny_keccak::{Hasher, Keccak};

use clap::Parser;
#[derive(Parser, Default, Debug)]
#[clap(name = "ZeroSeeker", about = "")]

pub struct Args {
    #[clap(short, long, help = "Set the entropy seed")]
    pub entropy_seed: String,
    #[clap(short, long, help = "Set the desired number of total zero bytes")]
    pub zero_bytes: u8,
    #[clap(short, long, help = "Set whether zero bytes are leading or total")]
    pub leading: bool,
}

fn main() {
    let args = Args::parse();
    if args.leading {
        println!(
            "Generating address with {} leading zero bytes...",
            args.zero_bytes
        );
    } else {
        println!(
            "Generating address with {} total zero bytes...",
            args.zero_bytes
        );
    }

    let lower_complexity: u8 = 2;

    // Run the search for the lower complexity value
    let start_time = Instant::now();
    mine_address_with_n_zero_bytes(&args.entropy_seed, lower_complexity, args.leading);
    let elapsed_time = start_time.elapsed();

    // Calculate the ratio between the probabilities for the lower and target complexity values
    let ratio = (1.0 / 256.0f64.powi(lower_complexity as i32))
        / (1.0 / 256.0f64.powi(args.zero_bytes as i32));

    // Estimate the time it would take to find a matching Ethereum address for the desired complexity value
    let estimated_time = elapsed_time.as_secs_f64() * ratio;

    let estimated_seconds = estimated_time as u64;
    let (days, hours, minutes, seconds) = {
        let (days, remainder) = (estimated_seconds / 86_400, estimated_seconds % 86_400);
        let (hours, remainder) = (remainder / 3_600, remainder % 3_600);
        let (minutes, seconds) = (remainder / 60, remainder % 60);
        (days, hours, minutes, seconds)
    };

    println!(
        "Estimated time to find an address with {} zero bytes: {} days, {} hours, {} minutes, and {} seconds",
        args.zero_bytes, days, hours, minutes, seconds
    );

    if let Some((private_key, contract_address)) =
        mine_address_with_n_zero_bytes(&args.entropy_seed, args.zero_bytes, args.leading)
    {
        let elapsed_time = start_time.elapsed();
        if args.leading {
            println!(
                "Found address with {} leading zero bytes in {} seconds: {:?}",
                args.zero_bytes,
                elapsed_time.as_secs(),
                contract_address
            );
        } else {
            println!(
                "Found address with {} zero bytes in {} seconds: {:?}",
                args.zero_bytes,
                elapsed_time.as_secs(),
                contract_address
            );
        }
        println!("Private key: 0x{private_key}");
    } else {
        println!("No matching address found.");
    }
}
