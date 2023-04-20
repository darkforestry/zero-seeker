# ZeroSeeker

ZeroSeeker is a Rust-based command-line utility that generates Ethereum contract addresses with a specified number of leading or total zero bytes. **Specialized for deployment of non-deterministic contracts**, ZeroSeeker generates a private key whose initial nonce can be used to generate a contract with chosen amount of leading or total zero bytes.

## Features

- Generates Ethereum addresses with a specified number of leading or total zero bytes in the contract address
- Parallelized search for faster results
- Estimates the time required to find an address with the desired complexity

## Usage

To use ZeroSeeker, run the following command:

```sh
cargo run --release -- -s "your_entropy_seed" -z number_of_zero_bytes [-l (true|false)]
```

Replace your_entropy_seed with a string that will be used as a seed for generating the private key, and number_of_zero_bytes with the desired number of zero bytes in the contract address. Use the -l flag followed by true or false to specify if you want to count leading zero bytes (true) or total zero bytes (false). If the -l flag is not provided, the default is false.

## Todo

- [ ] Reduce code shitiness
- [ ] GPU acceleration