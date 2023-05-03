mod io_reader;
use io_reader::IOReader;

mod solver;
pub mod util;



fn main() {
	let mut io_reader = IOReader::new();

	let input_digits = io_reader.read("Enter your digits, space-seperated: ");


	let input_digits = input_digits
		.trim() // clean up new lines
		.split(" ") // split by character
		.filter(|char| *char != "") // filter out empty characters because they're there
		.map(|char| char.parse::<u8>()) // parse character into u8
		.map(|num| num.unwrap_or(255)) // parse returns Result, unwrap. if unvalid, set to 255
		.filter(|num| *num < 10) // clean up values marked as invalid
		.collect::<Vec<u8>>(); // collect to vector


	if input_digits.len() != 4 {
		panic!("Invalid input!");
	}


	// let available_operations = io_reader.read("Enter available operations, or leave blank for all: ");

	// let available_operations = available_operations
	// 	.trim()
	// 	.split("")
	// 	.collect::<Vec<&str>>();

	// dbg!(available_operations);


	// let available_operations = vec![Operation::Add, Operation::Subtract, Operation::Multiply, Operation::Divide, Operation::Parentheses];
	// let available_operations = vec![String::from("+"), String::from("-"), String::from("*"), String::from("/"), String::from("()")];
	// let available_operations = EnabledOperations {addition: true, subtraction: true, multiplication: true, division: true, parentheses: false};
	let available_operations = String::from("+-*/");


	let solution = solver::brute_force(input_digits, available_operations);


	if solution.is_empty() {
		println!("No solution found!");
	} else {
		println!("Solution found!: {}", solution);
	}
}
