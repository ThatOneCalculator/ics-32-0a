fn main() {
	let num_vec = vec![2, 4, 6, 8];
	let sum: usize = num_vec.iter().sum();
	println!("{}", sum); // should be 20
}