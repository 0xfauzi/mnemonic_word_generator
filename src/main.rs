use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rand::{Rng, thread_rng};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::NonZeroU32;
use std::ops::BitAnd;
use std::path::Path;
use ring::{digest, pbkdf2};


const SEED_LENGTH: usize = digest::SHA512_OUTPUT_LEN;
pub type Seed = [u8; SEED_LENGTH];

fn main() {

    //Currently, all bip39 words are read from a local file
    let bip39_words = lines_from_file("src/english.txt");

    let private_key = create_random_sequence();

    let checksum = create_checksum(&private_key);
    let checksum_applied_to_private_key = append_checksum_to_random_sequence(&private_key, checksum);

    for i in 0..checksum_applied_to_private_key.len() {
        print!("{:#010b}", checksum_applied_to_private_key[i]);
    }

    let words = map_mnemonic_words(&checksum_applied_to_private_key, &bip39_words);
    println!("{:?}", words);

    println!("Generating 512-bit seed from mnemonic and salt");
    let mut words_as_string: String = String::new();
    for i in 0..words.len() - 1 {
        words_as_string.push_str(words.get(i).unwrap());
        words_as_string.push_str(" ");
    }
    words_as_string.push_str(words.get(words.len() - 1).unwrap());
    println!("{:?}", words_as_string);
    let seed = generate_seed_from_mnemonic(&words_as_string, "example");
    println!("Printing generated seed");
    println!("{:?}", seed);
}

fn create_random_sequence() -> [u8; 32] {
    thread_rng().gen::<[u8; 32]>()
}

fn create_checksum(random_sequence: &[u8; 32]) -> u8 {
    let hashed = hash_sha256(&random_sequence);
    let checksum = extract_first_8_bits(&hashed);
    checksum
}

fn append_checksum_to_random_sequence(random_sequence: &[u8; 32], checksum: u8) -> [u8; 33] {
    let mut checksum_applied: [u8; 33] = [0; 33];

    for i in 0..=31 { //indices from 0 to 31, inclusive
        checksum_applied[i] = random_sequence[i];
    }

    checksum_applied[checksum_applied.len() - 1] = checksum;

    checksum_applied
}

fn map_mnemonic_words(input: &[u8; 33], bip38_words: &Vec<String>) -> Vec<String> {

    let mut resulting_words: Vec<String> = vec![];

    let mut bits: Vec<u8> = vec![11; 0]; //each bit is stored as an unsigned 8 bit integer

    for i in 0..input.len() { // while there are u8s to read
        for j in (0..8).rev() { // j goes down from 8 because it's used in a bit mask to get the bit at position 8, 7, 6, 5 etc.
            let jth_bit = u8::bitand(input[i] >> j, 1); //right shift by j times and & 1, effectively getting the jth bit
            bits.push(jth_bit);
            if bits.len() > 0 && bits.len() % 11 == 0 { //if we have read our 11th bit, move on to constructing the next word
                let word_index = convert_bits_to_decimal(&bits) as usize;
                resulting_words.push(bip38_words.get(word_index).unwrap().parse().unwrap());
                bits.clear(); // we have our first 11 bits, reset and build another 11
            }
        }
    }

    resulting_words
}

//There is probably an idiomatic way to do this in Rust
fn convert_bits_to_decimal(bits: &Vec<u8>) -> u32 {
    let mut dec: u32 = 0;

    for i in 0..bits.len() {
        dec += (bits[i] as u32 * (2_u32.pow((bits.len() - 1 - i) as u32)) as u32) as u32;
    }

    dec
}

fn hash_sha256(input: &[u8; 32]) -> [u8; 32] {
    let mut sha_256_hasher = Sha256::new();
    sha_256_hasher.input(input);

    let mut output: [u8; 32] = [0; 32];
    sha_256_hasher.result(&mut output);

    output
}

fn extract_first_8_bits(input: &[u8; 32]) -> u8 {
    let result: u8 = input[0];
    result
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn generate_seed_from_mnemonic(mnemonic: &String, passphrase: &str) -> [u8; 64] {

    let salt = ("mnemonic".to_owned() + passphrase);

    let mut seed: Seed = [0u8; SEED_LENGTH];
    pbkdf2::derive(pbkdf2::PBKDF2_HMAC_SHA512, NonZeroU32::new(2048).unwrap(), &salt.as_bytes(), mnemonic.as_bytes(), &mut seed);
    seed
}