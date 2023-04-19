use clap::{App, Arg};
use sha3::{Digest, Sha3_256};

fn main() {
    let matches = App::new("ZeroSeeker")
        .arg(
            Arg::with_name("entropy_seed")
                .short("s")
                .long("seed")
                .value_name("ENTROPY_SEED")
                .help("Set the entropy seed")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("zero_bytes")
                .short("z")
                .long("zero-bytes")
                .value_name("ZERO_BYTES")
                .help("Set the desired number of total zero bytes")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let entropy_seed = matches.value_of("entropy_seed").unwrap();
    let zero_bytes: usize = matches
        .value_of("zero_bytes")
        .unwrap()
        .parse()
        .expect("Zero bytes must be a number");

    println!("Entropy seed: {}", entropy_seed);
    println!("Hashed entropy seed: {}", hash_entropy_seed(entropy_seed));
    println!("Zero bytes: {}", zero_bytes);
}

fn hash_entropy_seed(seed: &str) -> String {
    // Hash the random string using SHA3-256
    let mut hasher = Sha3_256::new();
    hasher.update(seed.as_bytes());
    let hash = hasher.finalize();

    // Return the hash as a hex-encoded string
    format!("{:x}", hash)
}
