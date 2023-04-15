pub fn print_help_message() {
	println!(" _____ _   _______ ");
	println!("|  ___| | / /_   _|");
	println!("| |__ | |/ /  | |  ");
	println!("|  __||    \\  | |  ");
	println!("| |___| |\\  \\ | |  ");
	println!("\\____/\\_| \\_/ \\_/  ");
	println!("");
	println!("ekt - Ethereum key toolkit");
	println!("Usage:");
	println!("-h\t Display help message");
	println!("-r *key amount*\t Generate a specific amount of random keys, default 1");
	println!("-t *key*\t Validate a private key, and get public key");
	println!("-v *thread count* *beginning*\t Generate vanity address");
}
