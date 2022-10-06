

mod wallet_lib;

fn main() {
    let (secret_key, public_key) = wallet_lib::generate_keypair();

    println!("secret key: {}", &secret_key.to_string());
    println!("public key: {}", &public_key.to_string());

    //calling the public key address and printing the result value
    //printing the address as a keccak256 hash
    let public_address = wallet_lib::public_key_address(&public_key);
    println!("public address: {:?}", public_address);
}
