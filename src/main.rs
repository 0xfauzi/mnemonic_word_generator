use std::str::FromStr;
use base58::ToBase58;
use rand::{Rng, thread_rng};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha2::{Sha256, Digest};
use ripemd::{Ripemd160};
use bitcoin::util::address::Address;
use bitcoin::network::constants::Network;

fn main() {
    let private_key = generate_private_key();
    println!("Private key: {}", hex::encode(&private_key));

    let public_key = generate_public_key(&private_key);
    println!("Public Key: {}", hex::encode(&public_key));

    let sha256_hashed_public_key = hash_sha256(&public_key);
    println!("Sha256 hashed public key: {}", hex::encode(&sha256_hashed_public_key));

    let ripemd160_hashed_public_key = hash_ripemd160(&sha256_hashed_public_key);
    println!("RIPEMD160 hashed public key: {}", hex::encode(&ripemd160_hashed_public_key));

    let mut result_with_version_byte = vec![0u8; 21];
    result_with_version_byte[1..].copy_from_slice(&ripemd160_hashed_public_key);
    println!("Result with version byte: {}", hex::encode(&result_with_version_byte));

    let hashed = hash_sha256(&result_with_version_byte);
    let hashed_2x = hash_sha256(&hashed);
    println!("Result with version byte and 2x sha256: {}", hex::encode(&hashed_2x));

    let base_58_check_encoded_address = base58check_encode(&hashed_2x, &result_with_version_byte);
    println!("Base58Check encoded address: {}", base_58_check_encoded_address);

    let actual_address = Address::from_str(&base_58_check_encoded_address);
    match actual_address {
        Ok(address) => println!("Generated address is valid {:?}", address),
        Err(error) => println!("Invalid address generated. {:?}", error.to_string()),
    };
}

fn generate_private_key() -> [u8; 32] {
    thread_rng().gen::<[u8; 32]>()
}

fn generate_public_key(private_key: &[u8; 32]) -> [u8; 33] {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(private_key).unwrap();
    let public_key = PublicKey::from_secret_key(&secp, &secret_key).serialize();
    public_key
}

fn hash_sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

fn hash_ripemd160(data: &[u8]) -> [u8; 20] {
    let mut hasher = Ripemd160::new();
    hasher.update(data);
    hasher.finalize().into()
}

fn base58check_encode(hashed_2x: &[u8; 32], result_with_version_byte: &[u8]) -> String {
    let checksum = &hashed_2x[0..4];
    let mut result = vec![0u8; 25];
    result[..21].copy_from_slice(result_with_version_byte);
    result[21..].copy_from_slice(checksum);
    result.to_base58()
}
