fn main() {
	let mut license = String::new();
	println!("Enter your license plate:");
	std::io::stdin().read_line(&mut license).unwrap();
	
	println!("{}", license);
}