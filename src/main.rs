mod io_reader;
use io_reader::IOReader;

mod solver;
pub mod util;



fn main() {
	let mut io_reader = IOReader::new();

	let input_digits = io_reader.read("Enter your digits, space-seperated: ");

	let input_digits = input_digits
		.trim()
		.chars()
		.map(|x| x.to_digit(10).unwrap_or(255) as u8)
		.filter(|num| *num < 10)
		.collect::<Vec<u8>>();


	if input_digits.len() != 4 {
		panic!("Invalid input!");
	}


	// let available_operations = vec![Operation::Add, Operation::Subtract, Operation::Multiply, Operation::Divide, Operation::Parentheses];
	// let available_operations = vec![String::from("+"), String::from("-"), String::from("*"), String::from("/"), String::from("()")];
	// let available_operations = EnabledOperations {addition: true, subtraction: true, multiplication: true, division: true, parentheses: false};

	// test value for available_operations: 3 1 3 4
	let available_operations = String::from("+-*/");


	let solution = solver::brute_force(input_digits, available_operations);


	if solution.is_empty() {
		println!("No solution found!");
	} else {
		println!("Solution found!: {}", solution);
	}
}
