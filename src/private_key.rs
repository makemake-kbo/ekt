use openssl::rand::rand_bytes;
use std::str::FromStr;
use ethers::prelude::*;
use ethers::types::Address;
use hex::encode;

// Generates 64 random bytes with openssl
pub fn random_bytes() -> String {
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
fn generate() {

	let bytes = random_bytes();
	let address = get_address_from_private_key(&bytes);

	println!("sk {}", bytes);
	println!("pk {:?}", address);
}

// Used for the -r argument
pub fn parse_arg_and_generate(args: Vec<String>) {
    if args.len() == 2 {
        generate();
        return;
    }
    // Generate args[2] ammount of sk's
    // If cannot parse generate 1
    let ammount: u32 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => 1,
    };
    for _ in 0..ammount {
        generate();
        println!("");
    }
    return;
}

// Validates SKs provided in the args
pub fn parse_arg_and_validate_sk(args: Vec<String>) {
    // Iterate over the items in the vector
    for (index, arg) in args.iter().enumerate().skip(2) {
        match arg.len() {
            64 => {
                println!("sk {}", arg);
                println!("pk {:?}\n", get_address_from_private_key(arg));
            },
            _ =>  println!("Invalid private key length at index {}", index),
        }
    }
}
