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

    pub fn save_to_file(&self, file_path: &str) -> Result<()> {
        let file = OpenOptions::new().write(true).create(true).open(file_path)?;
        let buf_writer = BufWriter::new(file);

        serde_json::to_writer_pretty(buf_writer, self)?;

        Ok(())
    }
    
    pub fn from_file(file_path: &str) -> Result<Wallet> {
        let file = OpenOptions::new().read(true).open(file_path)?;
        let buf_reader = BufReader::new(file);

        let wallet: Wallet = serde_json::from_reader(buf_reader)?;
        Ok(wallet)
    }

    pub fn get_secret_key(&self) -> Result<SecretKey> {
        let secret_key = SecretKey::from_str(&self.secret_key);
        Ok(secret_key)
    }

    pub fn get_public_key(&self) -> Result<PublicKey> {
        let pub_key = PublicKey::from_str(&self.public_key)?;
        Ok(pub_key)
    }
    
}