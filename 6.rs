fn main() {
	let mut valid = false;
	while !valid {
		let mut license = String::new();
		println!("Enter your license plate:");
		std::io::stdin().read_line(&mut license).unwrap();
		license = license.trim().to_string();
		let license_vec: Vec<char> = license.chars().collect();
		let mut hasnum = false;
		for i in license_vec.iter() {
			if i.is_numeric() { hasnum = true; }
		}
		if license.contains(char::is_whitespace) |
		!license.contains(char::is_alphanumeric) |
		!(license.to_uppercase() == license) |
		license_vec[0].is_numeric() |
		license_vec[license.len() - 1].is_numeric() |
		!hasnum {
			println!("Not valid, try again")
		}
		else {
			valid = true;
		}
	}
	println!("Valid");
}