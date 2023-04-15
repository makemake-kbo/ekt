mod private_key;
mod errors;
mod vanity;

use crate::private_key::get_address_from_private_key;
use crate::errors::print_help_message;
use crate::vanity::generate_vanity;

use std::env;
use tokio::sync::mpsc;
use tokio::task;

// Do not question the lack of match usage for the args
// I am lazy and Go gave me brainrot

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || args[1] == "-h" {
        print_help_message();
        return;
    } else if args[1] == "-r" {
        if args.len() == 2 {
            private_key::generate();
            return;
        }
        // Generate args[2] ammount of sk's
        // If cannot parse generate 1
        let ammount: u32 = match args[2].parse() {
            Ok(n) => n,
            Err(_) => 1,
        };
        for _ in 0..ammount {
            private_key::generate();
            println!("");
        }
        return;
    } else if args[1] == "-t" {
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
    } else if args[1] == "-v" {
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
    } else {
        print_help_message();
        return;
    }
}
