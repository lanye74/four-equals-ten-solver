// use std::io;

mod brute_forcer;
mod evaluator;
pub mod util;



fn main() {
	// let stdin = io::stdin();

	// let mut input = String::new();

	// println!("Enter your numbers: ");


	// stdin
	// 	.read_line(&mut input)
	// 	.expect("should've been able to read line!");


	// let input = input
	// 	.trim() // clean up new lines
	// 	.split("") // split by character
	// 	.filter(|char| *char != "") // filter out empty characters because they're there
	// 	.map(|char| char.parse::<u8>()) // parse character into u8
	// 	.map(|num| num.unwrap_or(255)) // parse returns result, unwrap. if unvalid, set to 255
	// 	.filter(|num| *num < 10) // clean up values marked as invalid
	// 	.collect::<Vec<u8>>(); // collect to vector


	// if input.len() != 4 {
	// 	panic!("Invalid input!");
	// }

	let mut input = vec![7, 4, 1, 1];


	let _solution = brute_forcer::brute_force(&mut input);
}
