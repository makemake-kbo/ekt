use crate::private_key::*;

use tokio::sync::mpsc;
use tokio::task;

use std::str::FromStr;
use ethers::prelude::*;

async fn generate_vanity(start: String) -> String {
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

// Spawn generate_vanity threads
pub async fn spawn_vanity_threads(args: Vec<String>) {
    let num_threads: usize = args[2].parse().unwrap();
    let (tx, mut rx) = mpsc::channel::<String>(num_threads);

    println!("Starting threads...");

    // Spawn threads
    for num in 0..num_threads {
        let tx = tx.clone();
        let start = args[3].clone();
        task::spawn(async move {
            let private_key = generate_vanity(start).await;
            tx.send(private_key).await.unwrap();
        });
        println!("Thread {} spawned, working...", num);
    }

    // Wait for the first thread to complete and print the result
    if let Some(private_key) = rx.recv().await {
        println!("sk {}", private_key);
        println!("pk {:?}\n", get_address_from_private_key(&private_key));
        std::process::exit(0);
    }
}
