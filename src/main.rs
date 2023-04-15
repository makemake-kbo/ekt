mod private_key;
mod errors;

use crate::private_key::get_address_from_private_key;
use crate::errors::print_help_message;
use std::env;

fn main() {
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
    } else if (args[1] == "-v") {
        println!("a");

    } else {
        print_help_message();
        return;
    }
}
