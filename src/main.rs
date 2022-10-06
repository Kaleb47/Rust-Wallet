use anyhow::Result;

mod wallet_lib;

fn main() -> Result<()> {
    let (secret_key, public_key) = wallet_lib::generate_keypair();

    println!("secret key: {}", &secret_key.to_string());
    println!("public key: {}", &public_key.to_string());

    //calling the public key address and printing the result value
    //printing the address as a keccak256 hash
    let public_address = wallet_lib::public_key_address(&public_key);
    println!("public address: {:?}", public_address);

    let crypto_wallet = wallet_lib::Wallet::new(&secret_key, &public_key);
    println!("crypto_wallet: {:?}", &crypto_wallet);

    let wallet_file_path = "crypto_wallet.json";
    crypto_wallet.save_to_file("crypto_wallet.json")?;

    let loaded_wallet = wallet_lib::Wallet::from_file(wallet_file_path)?;
    println!("loaded_wallet: {:?}", loaded_wallet);

    Ok(())
}
