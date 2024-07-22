# mnemonic_word_generator
[![Rust](https://github.com/0xfauzi/mnemonic_word_generator/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/0xfauzi/mnemonic_word_generator/actions/workflows/rust.yml)

Generates a mnemonic phrase according to BIP 39
# Bitcoin Mnemonic Phrase Generator

This Rust project implements a simple Bitcoin mnemonic phrase generator. It creates a random 12-word mnemonic phrase using the BIP39 wordlist, which can be used as a seed for generating Bitcoin wallets.

## Features

- Generates a random 12-word mnemonic phrase
- Uses the official BIP39 English wordlist
- Implements proper error handling
- Simple and easy to understand codebase

## Dependencies

This project uses the following external crates:

- `rand`: For generating cryptographically secure random numbers
- `rust-crypto`: For cryptographic functions
- `itertools`: For additional iterator functionality
- `ring`: For cryptographic operations

These dependencies are specified in the `Cargo.toml` file:

```toml
[dependencies]
rand = "0.8.4"
rust-crypto = "0.2.36"
itertools = "0.10.3"
ring = "0.16.20"
```

## Usage

To use this mnemonic phrase generator, you can call the `generate_mnemonic()` function. Here's an example of how to use it in your main function:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mnemonic = generate_mnemonic()?;
    println!("Generated mnemonic: {}", mnemonic);
    Ok(())
}
```

## Function Explanation

### `generate_mnemonic()`

This function generates a 12-word mnemonic phrase using the following steps:

1. It reads the BIP39 English wordlist from a file.
2. Generates 16 bytes (128 bits) of random data using the `rand` crate.
3. Converts the random data to a binary string.
4. Calculates a checksum using cryptographic functions from `rust-crypto` or `ring`.
5. Appends the checksum to the binary string.
6. Splits the resulting binary string into 12 segments of 11 bits each.
7. Converts each 11-bit segment to a decimal number.
8. Uses these numbers as indices to select words from the BIP39 wordlist.
9. Joins the selected words with spaces to form the final mnemonic phrase.

## Error Handling

The function returns a `Result` type, allowing for proper error handling. Any IO errors or unexpected issues during the mnemonic generation process will be captured and can be handled by the calling function.

## Security Considerations

While this implementation uses cryptographically secure random number generation via the `rand` crate and cryptographic functions from `rust-crypto` and `ring`, it's important to note that this is a basic implementation for educational purposes. For real-world applications involving cryptocurrency, it's crucial to use well-audited libraries and follow best practices for secure key management.

## Contributing

Contributions to improve the code or documentation are welcome. Please feel free to submit issues or pull requests to the repository.


## Disclaimer

This code is provided for educational purposes only. Use at your own risk. The authors and contributors are not responsible for any loss of funds or other damages that may occur from using this code.
