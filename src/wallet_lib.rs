use tiny_keccak::keccak256;
use web3::types::Address;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::io::BufWriter;
use std::str::FromStr;
use std::{fs::OpenOptions, io::BufReader};
use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};
 // generate a key pair
// this function has no input but returns a tuple of the Public key and private key
pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp = secp256k1::Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(111);
    secp.generate_keypair(&mut rng)
}

// this function takes a reference to the public key object, returning the public key object
pub fn public_key_address(public_key: &PublicKey) -> Address {
    let public_key  = public_key.serialize_uncompressed();

    debug_assert_eq!(public_key[0], 0x04);
    let hash = keccak256(&public_key[..]);

    Address::from_slice(&hash[12..])
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    pub secret_key: String,
    pub public_key: String,
    pub public_address: String,
}

impl Wallet {
    pub fn new(secret_key: &SecretKey, public_key: &PublicKey) -> Self {
        let addr: Address = public_key_address(&public_key);
        Wallet {
            secret_key: secret_key.to_string(),
            public_key: public_key.to_string(),
            public_address: format!("{:?}", addr),
        }
    }
}