pub fn print_help_message() {
	println!(" _____ _   _______ ");
	println!("|  ___| | / /_   _|");
	println!("| |__ | |/ /  | |  ");
	println!("|  __||    \\  | |  ");
	println!("| |___| |\\  \\ | |  ");
	println!("\\____/\\_| \\_/ \\_/  ");
	println!("");
	println!("ekt - Ethereum key tool");
	println!("Usage:");
	println!("-h\t Display help message");
	println!("-r *key amount*\t Generate a specific amount of random keys, default 1");
	println!("-t *key*\t Validate a private key, and get public key");
	println!("-v *thread count* *beginning*\t Generate vanity address");
	println!("\t Example:");
	println!("\t ekt -v 8 dead");
	println!("\t - Generates an address starting with dead using 8 threads");
}
