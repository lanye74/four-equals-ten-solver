use std::time::{Duration, Instant};

use crate::configurator::Config;
use super::evaluator;
use super::{OperatorPermutator, OperatorMapper};
use super::ParenthesesPermutator;



pub fn brute_force(config: &Config) -> BruteForcerOutput {
	let starting_time = Instant::now();


	// destructure
	let Config {
		ref input_digits,
		ref enabled_operations,

		target_number,

		find_all_solutions,
		solve_with_parentheses
	} = *config;


	let mut input = input_digits.clone();


	let input_len = input.len();

	println!("Generating number permutations...");
	let number_permutations = generate_permutations(&mut input);

	let mut solutions = vec![];
	let mut solutions_considered: u64 = 0;


	let operator_mapper = OperatorMapper::new(&enabled_operations);

	println!("Finding solutions...");

	// n! permutations, assuming no duplicates
	for number_permutation in number_permutations {
		let operator_permutator = OperatorPermutator::new(&operator_mapper, input_len - 1);

		// (# operators enabled)^n permutations
		// for low values of n, this ordering of the loops is less efficient; but more efficient for higher values
		for operator_permutation in operator_permutator {
			solutions_considered += 1;

			let expression = build_expression(&number_permutation, &operator_permutation, None, None);

			let result = evaluator::evaluate(&expression);

			if result == target_number {
				// winner found!
				solutions.push(expression);

				if find_all_solutions == false {
					return BruteForcerOutput {
						solutions,
						solutions_considered,

						time_taken: starting_time.elapsed()
					};
				}
			}


			if solve_with_parentheses == true {
				let parentheses_permutator = ParenthesesPermutator::new(input_len);

				// (n - 1) + (n - 2) + ... permutations while (n - x) != 0
				for (lparen_pos, rparen_pos) in parentheses_permutator {
					solutions_considered += 1;

					let expression = build_expression(&number_permutation, &operator_permutation, Some(lparen_pos), Some(rparen_pos));

					let result = evaluator::evaluate(&expression);


					if result == target_number {
						solutions.push(expression);

						if find_all_solutions == false {
							return BruteForcerOutput {
								// me when 8 layers of nesting
								solutions,
								solutions_considered,

								time_taken: starting_time.elapsed()
							};
						}
					}
				}
			}
		}
	}


	return BruteForcerOutput {
		solutions,
		solutions_considered,

		time_taken: starting_time.elapsed()
	};
}



fn build_expression(number_permutation: &Vec<u8>, operator_permutation: &Vec<char>, lparen_pos: Option<usize>, rparen_pos: Option<usize>) -> String {
	let input_len = number_permutation.len();

	let lparen_pos = lparen_pos.unwrap_or(input_len + 1);
	let rparen_pos = rparen_pos.unwrap_or(input_len + 1);

	let mut expression_builder = String::new();

	// build expression
	for i in 0..input_len {
		if i == lparen_pos {
			expression_builder.push('(');
		}

		expression_builder.push(char::from_digit(number_permutation[i] as u32, 10).unwrap());

		if i == rparen_pos {
			expression_builder.push(')');
		}

		// ensures that a dangling operator isn't placed
		if i != input_len - 1 {
			expression_builder.push(operator_permutation[i]);
		}
	}

	return expression_builder;
}



fn generate_permutations(input: &mut Vec<u8>) -> Vec<Vec<u8>> {
	let input_len = input.len();

	let mut output: Vec<Vec<u8>> = vec![];
	let mut state: Vec<usize> = vec![0; input_len];

	output.push(input.clone());

	let mut pointer = 1;

	// quite honestly i have no idea how this works i just ripped it from wikipedia (heap's algorithm)
	// edit: i now have a slightly better idea how it works after looking at the quickperm algorithms
	while pointer < input_len {
		if state[pointer] < pointer {
			// neat branchless trick; if pointer is even {0} else {state[pointer]}
			let pointer_2 = (pointer % 2) * state[pointer];

			input.swap(pointer, pointer_2);

			output.push(input.clone());

			state[pointer] += 1;

			pointer = 1;
		} else {
			state[pointer] = 0;
			pointer += 1;
		}
	}


	output.sort();
	output.dedup();

	return output;
}



#[cfg(test)]
#[test]
fn test_brute_forcer() {
	let config_1 = Config {
		input_digits: vec![8, 2, 7, 1],
		enabled_operations: String::from("+-*/"),

		target_number: 10.0,

		find_all_solutions: false,
		solve_with_parentheses: false
	};


	let mut computation_1 = brute_force(&config_1);
	assert_eq!(evaluator::evaluate(&computation_1.solutions.pop().unwrap()), 10.0);


	let config_2 = Config {
		input_digits: vec![5, 1, 6, 3],
		enabled_operations: String::from("+-*/"),

		target_number: 10.0,

		find_all_solutions: false,
		solve_with_parentheses: false
	};

	let mut computation_2 = brute_force(&config_2);
	assert_eq!(evaluator::evaluate(&computation_2.solutions.pop().unwrap()), 10.0);



	// with parentheses

	let config_3 = Config {
		input_digits: vec![9, 9, 1, 1],
		enabled_operations: String::from("+-*/"),

		target_number: 10.0,

		find_all_solutions: false,
		solve_with_parentheses: true
	};

	let mut computation_3 = brute_force(&config_3);
	assert_eq!(evaluator::evaluate(&computation_3.solutions.pop().unwrap()), 10.0);


	let config_4 = Config {
		input_digits: vec![5, 1, 1, 1],
		enabled_operations: String::from("+-*/"),

		target_number: 10.0,

		find_all_solutions: false,
		solve_with_parentheses: true
	};

	let mut computation_4 = brute_force(&config_4);
	assert_eq!(evaluator::evaluate(&computation_4.solutions.pop().unwrap()), 10.0);


	// with disabled operations

	let config_5 = Config {
		input_digits: vec![2, 5, 1, 1],
		enabled_operations: String::from("*/"),

		target_number: 10.0,

		find_all_solutions: false,
		solve_with_parentheses: false
	};

	let mut computation_5 = brute_force(&config_5);
	assert_eq!(evaluator::evaluate(&computation_5.solutions.pop().unwrap()), 10.0);


	// with different target

	let config_6 = Config {
		input_digits: vec![4, 9, 5, 2],
		enabled_operations: String::from("+-*/"),

		target_number: 11.0,

		find_all_solutions: false,
		solve_with_parentheses: true // this actually requires parentheses
	};

	let mut computation_6 = brute_force(&config_6);
	assert_eq!(evaluator::evaluate(&computation_6.solutions.pop().unwrap()), 11.0);
}



pub struct BruteForcerOutput {
	pub solutions: Vec<String>,
	pub solutions_considered: u64,

	pub time_taken: Duration
}
