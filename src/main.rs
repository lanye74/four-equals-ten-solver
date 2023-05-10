mod io_reader;
use io_reader::IOReader;

mod solver;



fn main() {
	let mut io_reader = IOReader::new();

	// this can literally be put in any format as long as there's 4 digits
	let input_digits = io_reader.read("Enter your digits: ");

	let input_digits = input_digits
		.trim()
		.chars()
		.map(|x| x.to_digit(10).unwrap_or(255) as u8)
		.filter(|num| *num < 10)
		.collect::<Vec<u8>>();


	if input_digits.len() != 4 {
		panic!("{} digits!",
			if input_digits.len() < 4 {"Not enough"} else {"Too many"}
		);
	}


	let find_all_solutions = io_reader.read("Do you want to find all solutions? Y/N: ");
	let find_all_solutions = find_all_solutions.trim().to_ascii_lowercase();

	let find_all_solutions = match find_all_solutions.as_str() {
		"y" => true,
		"n" => false,
		_ => panic!("Invalid input!")
	};


	// let available_operations = vec![Operation::Add, Operation::Subtract, Operation::Multiply, Operation::Divide, Operation::Parentheses];
	// let available_operations = vec![String::from("+"), String::from("-"), String::from("*"), String::from("/"), String::from("()")];
	// let available_operations = EnabledOperations {addition: true, subtraction: true, multiplication: true, division: true, parentheses: false};

	// test value for available_operations: 3 1 3 4
	let available_operations = String::from("+-*/");


	let solve_with_parentheses = true;


	let solutions = solver::brute_force(input_digits, available_operations, find_all_solutions, solve_with_parentheses);


	if solutions.is_empty() {
		println!("No solution found!");
	} else {
		println!("Solution{} found!:",
			if solutions.len() > 1 {"s"} else {""}
		);

		for sol in solutions {
			println!("{}", sol);
		}
	}
}



// #[allow(dead_code)]
// struct Config {
// 	digits: Vec<u8>,
// 	operations: String,

// 	find_all_solutions: bool,
// 	solve_with_parentheses: bool
// }
