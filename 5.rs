fn main() {
	let string_vec = vec!["not", "gonna", "be", "the", "largest", "word"];
	let mut num_vec: Vec<usize> = vec![];
	for i in string_vec.iter() {
		num_vec.push(i.len());
	}
	let max_size = num_vec.iter().max().unwrap();
	let index_of_max_size = num_vec.iter().position(|&r| r == *max_size).unwrap();
	let largest_word = string_vec.get(index_of_max_size).unwrap();
	println!("{}", largest_word); // should be "largest"
}