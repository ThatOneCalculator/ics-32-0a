fn main() {
	let part1 = "Hello";
	let part2 = " UCI";
	let result = [part1, part2].join("");
	print!("{}", result.trim()); // should be "Hello UCI"
}