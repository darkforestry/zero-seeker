use ethers::types::H160;
use libsecp256k1::{PublicKey, SecretKey};
use rayon::prelude::*;
use rlp::RlpStream;
use rug::{ops::Pow, Float};
use sha3::{Digest, Keccak256, Sha3_256};
use std::error::Error;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tiny_keccak::{Hasher, Keccak};

pub fn find_optimal_batch_size(entropy_seed: &str, leading: bool) -> u64 {
    let mut best_time = Duration::from_secs(1000000);
    let mut best_batch_size = 10;
    let mut current_batch_size = best_batch_size;

    loop {
        let start = Instant::now();
        mine_address_with_n_zero_bytes(entropy_seed, 2, leading, current_batch_size);
        let duration = start.elapsed();

        if duration < best_time {
            best_time = duration;
            best_batch_size = best_batch_size * 2;
            current_batch_size = best_batch_size;
        } else {
            current_batch_size = (current_batch_size as f64 / 1.5) as u64;
            if current_batch_size < best_batch_size {
                return best_batch_size;
            }
        }
    }
}

pub fn mine_address_with_n_zero_bytes(
    entropy_seed: &str,
    zero_bytes: u8,
    leading: bool,
    batch_size: u64,
) -> Option<(String, H160)> {
    let counter = Arc::new(AtomicU64::new(0));

    loop {
        let result = (0..batch_size).into_par_iter().find_map_any(|_| {
            let mut address_buf = H160::default(); // Allocate the address buffer once, outside the loop
            let mut contract_address_buf = H160::default(); // Allocate the contract address buffer once, outside the loop

            counter.fetch_add(batch_size, Ordering::SeqCst);
            let private_key =
                hash_entropy_seed(entropy_seed, counter.load(Ordering::Relaxed) as u128);
            address_from_private_key(&private_key, &mut address_buf).unwrap();
            contract_address_from_sender(&address_buf, &mut contract_address_buf);

            let zero_byte_count = if leading {
                count_leading_zero_bytes(&contract_address_buf)
            } else {
                count_zero_bytes(&contract_address_buf)
            };

            if zero_byte_count >= zero_bytes {
                Some((hex::encode(private_key), contract_address_buf))
            } else {
                None
            }
        });

        if let Some(address_with_n_zero_bytes) = result {
            return Some(address_with_n_zero_bytes);
        }
    }
}
pub fn hash_entropy_seed(seed: &str, counter: u128) -> [u8; 32] {
    // Hash the random string using SHA3-256
    let mut hasher = Sha3_256::new();
    hasher.update(seed.as_bytes());
    hasher.update(counter.to_le_bytes()); // Add the counter to the input data
    hasher.finalize().into()
}

pub fn address_from_private_key(
    private_key: &[u8; 32],
    address_buf: &mut H160,
) -> Result<(), Box<dyn Error>> {
    // Create a SecretKey
    let secret_key = SecretKey::parse_slice(private_key)?;

    // Derive the PublicKey from the SecretKey
    let public_key = PublicKey::from_secret_key(&secret_key);

    // Serialize the compressed public key
    let serialized_pubkey = public_key.serialize();

    // Hash the public key using Keccak-256
    let mut hasher = Keccak::v256();
    let mut hash = [0u8; 32];
    hasher.update(&serialized_pubkey[1..]); // Skip the first byte, as it only indicates the format
    hasher.finalize(&mut hash);

    // Retrieve the Ethereum address from the hash
    address_buf.assign_from_slice(&hash[12..]);

    Ok(())
}

pub fn contract_address_from_sender(sender: &H160, contract_address_buf: &mut H160) {
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
    contract_address_buf.assign_from_slice(&hash.as_slice()[12..])
}

pub fn count_zero_bytes(address: &H160) -> u8 {
    let mut count = 0;
    for b in address.0.iter() {
        if *b == 0 as u8 {
            count += 1
        }
    }
    count
}

pub fn count_leading_zero_bytes(address: &H160) -> u8 {
    let mut count = 0;
    for b in address.0.iter() {
        if *b == 0 as u8 {
            count += 1
        } else {
            break;
        }
    }
    count
}

pub fn expected_attempts(zero_bytes: u64, leading: bool) -> f64 {
    let total_bytes = 20u64;
    let probability_zero = Float::with_val(53, 1) / Float::with_val(53, 256);
    let probability_non_zero = Float::with_val(53, 1) - &probability_zero;

    if leading {
        let probability = probability_zero.pow(zero_bytes as i32);
        1.0 / probability.to_f64()
    } else {
        let mut total_probability = Float::with_val(53, 0);

        for combination in 0..combinations(total_bytes, zero_bytes) {
            let probability = Float::with_val(53, combination)
                * probability_zero.clone().pow(zero_bytes as i32)
                * Float::with_val(53, probability_non_zero.clone())
                    .pow((total_bytes - zero_bytes) as i32);
            total_probability += probability;
        }

        1.0 / total_probability.to_f64()
    }
}

fn combinations(n: u64, k: u64) -> u64 {
    factorial(n) / (factorial(k) * factorial(n - k))
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        (1..=n).product()
    }
}
