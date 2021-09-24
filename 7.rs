fn main() {
	let string_to_search = "I'd just like to interject for a moment. What you're refering to as Linux, is in fact, GNU/Linux, or as I've recently taken to calling it, GNU plus Linux.";
	let char_to_search: char = 'r';
	let char_vec: Vec<char> = string_to_search.chars().collect();
	let mut count = 0;
	for i in char_vec {
		if i == char_to_search {
			count += 1;
		}
	}
	println!("{}", count); // should be 7
}