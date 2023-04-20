use clap::{App, Arg};
use ethers::types::H160;
use libsecp256k1::{PublicKey, SecretKey};
use rlp::RlpStream;
use sha3::{Digest, Keccak256, Sha3_256};
use std::error::Error;
use tiny_keccak::{Hasher, Keccak};

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
    println!(
        "Address from private key: {:?}",
        address_from_private_key(&hash_entropy_seed(entropy_seed)).unwrap()
    );
    println!(
        "Contract address from sender: {:?}",
        contract_address_from_sender(
            &address_from_private_key(&hash_entropy_seed(entropy_seed)).unwrap()
        )
    );
    println!("Zero bytes: {}", zero_bytes);
    println!(
        "Count zero bytes: {}",
        count_zero_bytes(
            &"0x00f9c3839cd99643aeb665093520f900461ee100"
                .parse::<H160>()
                .unwrap()
        )
    )
}

fn hash_entropy_seed(seed: &str) -> String {
    // Hash the random string using SHA3-256
    let mut hasher = Sha3_256::new();
    hasher.update(seed.as_bytes());
    let hash = hasher.finalize();

    // Return the hash as a hex-encoded string
    format!("{:x}", hash)
}

fn address_from_private_key(private_key: &str) -> Result<H160, Box<dyn Error>> {
    // Parse the private key string to bytes and create a SecretKey
    let private_key_bytes = hex::decode(private_key)?;
    let secret_key = SecretKey::parse_slice(&private_key_bytes)?;

    // Derive the PublicKey from the SecretKey
    let public_key = PublicKey::from_secret_key(&secret_key);

    // Serialize the compressed public key
    let serialized_pubkey = public_key.serialize();

    // Hash the public key using Keccak-256
    let mut hasher = Keccak::v256();
    let mut hash = [0u8; 32];
    hasher.update(&serialized_pubkey[1..]); // Skip the first byte, as it only indicates the format
    hasher.finalize(&mut hash);

    // Calculate the Ethereum address from the hash
    let mut address = H160::default();
    address.assign_from_slice(&hash[12..]);

    Ok(address)
}

fn contract_address_from_sender(sender: &H160) -> H160 {
    // RLP encode the sender address and a nonce of 0
    let mut rlp_stream = RlpStream::new_list(2);
    rlp_stream.append(&sender.as_bytes());
    rlp_stream.append(&0u64);

    let encoded = rlp_stream.out();

    // Hash the RLP encoded data using Keccak-256
    let mut hasher = Keccak256::new();
    hasher.update(encoded);
    let hash = hasher.finalize();

    // The last 20 bytes of the hash are the contract address
    H160::from_slice(&hash.as_slice()[12..])
}

fn count_zero_bytes(address: &H160) -> usize {
    address.as_bytes().iter().filter(|&byte| *byte == 0).count()
}
