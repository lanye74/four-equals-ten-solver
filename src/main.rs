use std::io;

mod brute_forcer;
pub mod parser;



fn main() {
	let stdin = io::stdin();

	let mut input = String::new();

	println!("Enter your numbers: ");


	stdin
		.read_line(&mut input)
		.expect("should've been able to read line!");


	let input = input
		.trim() // clean up new lines
		.split("") // split by character
		.filter(|x| *x != "") // filter out empty characters because they're there
		.map(|x| x.parse::<u8>()) // parse character into u8
		.map(|x| x.unwrap_or(255)) // parse returns result, unwrap. if unvalid, set to 255
		.filter(|x| *x < 10) // cleanup values marked as invalid
		.collect::<Vec<u8>>(); // collect to vector


	if input.len() != 4 {
		panic!("Invalid input!");
	}


	let solution = brute_forcer::brute_force(&input);
}
