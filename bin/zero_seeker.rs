use std::time::Instant;

use clap::Parser;
use zero_seeker::find_optimal_batch_size;
#[derive(Parser, Default, Debug)]
#[clap(name = "ZeroSeeker", about = "")]

pub struct Args {
    #[clap(
        short,
        long,
        help = "Set the entropy seed, which must be at least 32 characters long"
    )]
    pub entropy_seed: String,
    #[clap(short, long, help = "Set the desired number of total zero bytes")]
    pub zero_bytes: u8,
    #[clap(short, long, help = "Set whether zero bytes are leading or total")]
    pub leading: bool,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    if args.entropy_seed.len() < 32 {
        return Err(format!(
            "The entropy seed must be at least 32 characters long.",
        ));
    }

    let lower_complexity: u8;
    if args.leading {
        lower_complexity = 2;
        println!(
            "Generating address with {} leading zero bytes...",
            args.zero_bytes
        );
    } else {
        lower_complexity = 3;
        println!(
            "Generating address with {} total zero bytes...",
            args.zero_bytes
        );
    }

    // Run the search for the lower complexity value
    let start_time = Instant::now();
    let batch_size = 200;
    zero_seeker::mine_address_with_n_zero_bytes(
        &args.entropy_seed,
        lower_complexity,
        args.leading,
        batch_size,
    );
    let elapsed_time = start_time.elapsed();

    // Calculate the ratio between the probabilities for the lower and target complexity values
    let ratio = zero_seeker::expected_attempts(args.zero_bytes as u64, args.leading)
        / zero_seeker::expected_attempts(lower_complexity as u64, args.leading);

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

    if let Some((private_key, contract_address)) = zero_seeker::mine_address_with_n_zero_bytes(
        &args.entropy_seed,
        args.zero_bytes,
        args.leading,
        batch_size,
    ) {
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

    Ok(())
}
