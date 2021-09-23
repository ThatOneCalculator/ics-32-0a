use std::f64;

pub fn theorem<T: Into<f64>>(adjacent: T, opposite: T) -> f64 {
	let hypotenuse: f64 = adjacent.into().powi(2) + opposite.into().powi(2);
	return hypotenuse.sqrt();
}

fn main() {
	let adjacent: f64 = 5.0;
	let opposite: f64 = 7.0;
	let hypotenuse = theorem(adjacent, opposite);
	let perimiter = adjacent + opposite + hypotenuse;
	println!("{}", perimiter);
}