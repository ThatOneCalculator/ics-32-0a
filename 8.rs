fn main() {
	let palindrome = "avid diva";
	let reverse_palindrome = palindrome.chars().rev().collect::<String>();
	let result = if palindrome == reverse_palindrome{ "is" } else { "is not" };
	println!("{} {} a palindrome.", palindrome, result); // should be "avid diva is a palindrome."
}