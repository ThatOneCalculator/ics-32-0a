fn main() {
	let mut name = String::new();
	let mut age = String::new();
	println!("Enter your name:");
	std::io::stdin().read_line(&mut name).unwrap();
	println!("Enter your age:");
	std::io::stdin().read_line(&mut age).unwrap();
	let int_age = age.trim().parse::<usize>().unwrap();
	let years = if int_age == 1 { "year" } else { "years" };
	println!("Your name is {} and you are {} {} old.", name.trim(), age.trim(), years);
}