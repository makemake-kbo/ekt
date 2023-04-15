use crate::private_key::*;

use std::str::FromStr;
use ethers::prelude::*;

pub async fn generate_vanity(start: String) -> String {
	let mut sk = random_bytes();
	let mut wallet = Wallet::from_str(&sk).unwrap();
	let mut combined = start.clone();

	// Check if start comes with 0x prefix
	if !start.starts_with("0x") {
		// Add 0x prefix
		combined = format!("{}{}", "0x", start);
	}

	// While addr does not begin with start generate new key
	while !wallet.address().to_string().starts_with(&combined) {
		sk = random_bytes();
		wallet = Wallet::from_str(&sk).unwrap();
	}

	sk
}