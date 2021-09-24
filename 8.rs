fn main() {
	let palindrome = "RACECAR";
	let reverse_palindrome = palindrome.chars().rev().collect::<String>();
	let result = if palindrome == reverse_palindrome{ "is" } else { "is not" };
	println!("{} {} a palindrome.", palindrome, result); // should be "RACECAR is a palindrome."
}