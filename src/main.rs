mod private_key;
mod errors;
mod vanity;

use crate::private_key::*;
use crate::errors::print_help_message;
use crate::vanity::spawn_vanity_threads;

use std::env;

// Do not question the lack of match usage for the args
// I am lazy and Go gave me brainrot

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // If no arguments provided, print error
    if args.len() == 1 {
        print_help_message();
        return;
    }
    match args[1].as_str() {
        "-h" => print_help_message(),
        "-r" => parse_arg_and_generate(args),
        "-t" => parse_arg_and_validate_sk(args),
        "-v" => spawn_vanity_threads(args).await,
        _ => print_help_message(),
    }
}
