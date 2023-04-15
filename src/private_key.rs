use openssl::rand::rand_bytes;
use std::str::FromStr;
#[allow(unused_imports)]
use ethers::prelude::*;
use ethers::types::Address;

use hex::encode;

// Generates 64 random bytes with openssl
fn random_bytes() -> String {
	let mut bytes = [0u8; 32];
	rand_bytes(&mut bytes).unwrap();
	
	encode(bytes)
}

// Generates an address with ethers
pub fn get_address_from_private_key(private_key: &String) -> Address {
    // Create new wallet from private_key
    let wallet = Wallet::from_str(private_key).unwrap();
    // Get address from wallet
    let address = wallet.address();

    address

}

// Prints out random bytes and associated address
pub fn generate() {

	let bytes = random_bytes();
	let address = get_address_from_private_key(&bytes);

	println!("sk {}", bytes);
	println!("pk {:?}", address);
}
