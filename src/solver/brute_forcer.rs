use std::time::{Duration, Instant};

use crate::configurator::Config;

use super::evaluator;
use super::{OperatorPermutator, OperatorMapper};
use super::ParenthesesPermutator;
use super::tokenizer;

use super::UnsafePush;



pub struct BruteForcerOutput {
	pub solutions: Vec<String>,
	pub solutions_considered: u64,

	pub time_taken: Duration
}



pub fn brute_force(config: &Config) -> BruteForcerOutput {
	let starting_time = Instant::now();


	// destructure
	let Config {
		ref input_digits,
		ref enabled_operations,

		target_number,

		find_all_solutions,
		solve_with_parentheses,

		..
	} = *config;


	let mut input = input_digits.clone();
	let input_len = input.len();

	println!("Generating number permutations...");
	let number_permutations = generate_permutations(&mut input);

	let mut solutions = vec![];
	let mut solutions_considered: u64 = 0;


	let operator_mapper = OperatorMapper::new(&enabled_operations);

	println!("Finding solutions...");

	// number permutation len + operator permutation len (which is equal to number permutation len, -1)
	let mut expression_builder_parenless = String::with_capacity(number_permutations[0].len() * 2 - 1);
	let mut tokens_vec_parenless = Vec::with_capacity(number_permutations[0].len() * 2 - 1);

	// +2 for paren
	let mut expression_builder_with_paren = String::with_capacity(number_permutations[0].len() * 2 - 1 + 2);
	let mut tokens_vec_with_paren = Vec::with_capacity(number_permutations[0].len() * 2 - 1 + 2);

	// having two of each of these is faster than one, probably due to their differing capacity


	// n! permutations, assuming no duplicates
	for number_permutation in number_permutations {
		let operator_permutator = OperatorPermutator::new(&operator_mapper, input_len - 1);

		// (# operators enabled)^(n - 1) permutations
		// for low values of n, this ordering of the loops is less efficient; but more efficient for higher values
		for operator_permutation in operator_permutator {
			solutions_considered += 1;

			build_expression_into(&mut expression_builder_parenless, &number_permutation, &operator_permutation);

			tokenizer::tokenize_into(&mut tokens_vec_parenless, &expression_builder_parenless);

			let result = evaluator::evaluate_tokens(&mut tokens_vec_parenless);


			if result == target_number {
				// winner found!
				solutions.push(expression_builder_parenless.clone());

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
				for paren_pos in parentheses_permutator {
					solutions_considered += 1;

					// possibly pass paren_pos by ref, though it will require more dereferencing
					build_expression_with_paren_into(&mut expression_builder_with_paren, &number_permutation, &operator_permutation, paren_pos);

					tokenizer::tokenize_into(&mut tokens_vec_with_paren, &expression_builder_with_paren);

					let result = evaluator::evaluate_tokens(&mut tokens_vec_with_paren);


					if result == target_number {
						solutions.push(expression_builder_with_paren.clone());

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



fn build_expression_into(expression_builder: &mut String, number_permutation: &[u8], operator_permutation: &[char]) {
	expression_builder.clear();

	let input_len = number_permutation.len();

	for i in 0..input_len {
		expression_builder.unsafe_push(char::from_digit(number_permutation[i] as u32, 10).unwrap());

		// ensures that a dangling operator isn't placed
		if i != input_len - 1 {
			expression_builder.unsafe_push(operator_permutation[i]);
		}
	}
}



fn build_expression_with_paren_into(expression_builder: &mut String, number_permutation: &[u8], operator_permutation: &[char], (lparen_pos, rparen_pos): (usize, usize)) {
	expression_builder.clear();

	let input_len = number_permutation.len();

	// build expression
	for i in 0..input_len {
		if i == lparen_pos {
			expression_builder.unsafe_push('(');
		}

		expression_builder.unsafe_push(char::from_digit(number_permutation[i] as u32, 10).unwrap());

		if i == rparen_pos {
			expression_builder.unsafe_push(')');
		}

		// ensures that a dangling operator isn't placed
		if i != input_len - 1 {
			expression_builder.unsafe_push(operator_permutation[i]);
		}
	}
}



fn generate_permutations(input: &mut Vec<u8>) -> Vec<Vec<u8>> {
	let input_len = input.len();

	let mut output: Vec<Vec<u8>> = vec![input.clone()];
	let mut state: Vec<usize> = vec![0; input_len];

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

	// remove duplicate numbers
	input.sort();
	input.dedup();

	// only waste time sorting/deduping the output if there are duplicates in the input
	if input.len() != input_len {
		output.sort();
		output.dedup();
	}


	// TODO: this really should return a flatmap with a data structure providing iterator/index methods
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
		solve_with_parentheses: false,

		no_print_solutions: false
	};


	let mut computation_1 = brute_force(&config_1);
	assert_eq!(evaluator::evaluate_string(&computation_1.solutions.pop().unwrap()), 10.0);


	let config_2 = Config {
		input_digits: vec![5, 1, 6, 3],
		enabled_operations: String::from("+-*/"),
		target_number: 10.0,

		find_all_solutions: false,
		solve_with_parentheses: false,

		no_print_solutions: false
	};

	let mut computation_2 = brute_force(&config_2);
	assert_eq!(evaluator::evaluate_string(&computation_2.solutions.pop().unwrap()), 10.0);


	// with parentheses

	let config_3 = Config {
		input_digits: vec![9, 9, 1, 1],
		enabled_operations: String::from("+-*/"),
		target_number: 10.0,

		find_all_solutions: false,
		solve_with_parentheses: true,

		no_print_solutions: false
	};

	let mut computation_3 = brute_force(&config_3);
	assert_eq!(evaluator::evaluate_string(&computation_3.solutions.pop().unwrap()), 10.0);


	let config_4 = Config {
		input_digits: vec![5, 1, 1, 1],
		enabled_operations: String::from("+-*/"),
		target_number: 10.0,

		find_all_solutions: false,
		solve_with_parentheses: true,

		no_print_solutions: false
	};

	let mut computation_4 = brute_force(&config_4);
	assert_eq!(evaluator::evaluate_string(&computation_4.solutions.pop().unwrap()), 10.0);


	// with disabled operations

	let config_5 = Config {
		input_digits: vec![2, 5, 1, 1],
		enabled_operations: String::from("*/"),
		target_number: 10.0,

		find_all_solutions: false,
		solve_with_parentheses: false,

		no_print_solutions: false
	};

	let mut computation_5 = brute_force(&config_5);
	assert_eq!(evaluator::evaluate_string(&computation_5.solutions.pop().unwrap()), 10.0);


	// with different target

	let config_6 = Config {
		input_digits: vec![4, 9, 5, 2],
		enabled_operations: String::from("+-*/"),
		target_number: 11.0,

		find_all_solutions: false,
		solve_with_parentheses: true, // this actually requires parentheses

		no_print_solutions: false
	};

	let mut computation_6 = brute_force(&config_6);
	assert_eq!(evaluator::evaluate_string(&computation_6.solutions.pop().unwrap()), 11.0);
}
