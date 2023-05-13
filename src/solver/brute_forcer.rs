use std::collections::HashSet;

use crate::configurator::Config;
use crate::solver::evaluator;
use super::OperatorPermutator;
use super::ParenthesesPermutator;



pub fn brute_force(config: Config) -> Vec<String> {
	// destructure
	let Config {
		input_digits: mut input,
		enabled_operations,

		find_all_solutions,
		solve_with_parentheses
	} = config;


	let input_len = input.len();

	println!("Generating permutations...");

	let number_permutations = generate_permutations(&mut input);

	let mut operator_permutator = OperatorPermutator::new(enabled_operations, input_len - 1);

	let mut output = vec![];


	// attempt to solve without parentheses
	println!("Finding solutions{}", if solve_with_parentheses == false {"..."} else {" without parentheses..."});

	#[allow(unused_labels)]
	'number_permutations: for number_permutation in number_permutations.iter() {
		operator_permutator.reset();

		'operation_permutations: loop {
			let mut expression_builder = String::new();

			// build expression
			for i in 0..input_len {
				expression_builder.push(char::from_digit(number_permutation[i] as u32, 10).unwrap());

				// ensures that a dangling operator isn't placed
				if i != input_len - 1 {
					expression_builder.push(operator_permutator.get_operator_at(i));
				}
			}

			let result = evaluator::evaluate(expression_builder.clone());

			if result == 10.0 {
				// winner found!
				output.push(expression_builder);

				if find_all_solutions == false {
					// break 'number_permutations;

					return output;
				}
			}


			operator_permutator.increment();

			if operator_permutator.is_maxed == true {
				// worked through every operator combination; run the loop again
				// this is equivalent to continue 'permutation loop but just for clarity

				break 'operation_permutations;
			}
		}
	}


	if solve_with_parentheses == true {
		println!("Finding solutions with parentheses...");

		// i may be atheist but god save me

		// first step: figure out how to store parentheses locations

		// [before] 0, [after] 1?
		// (1+2)+3+4
		// [before] 2, [after] 3
		// 1+2+(3+4)
		// this should be fine

		let mut parentheses_permutator = ParenthesesPermutator::new(input_len);

		// technically redundant but it makes me feel better
		operator_permutator.reset();

		'parentheses_permutations: loop {
			let (lparen_pos, rparen_pos) = parentheses_permutator.get_state();

			#[allow(unused_labels)]
			'number_permutations: for number_permutation in number_permutations.iter() {
				operator_permutator.reset();

				'operation_permutations: loop {
					let mut expression_builder = String::new();

					for i in 0..input_len {
						// HELP

						if i == lparen_pos {
							// me when 7 layers of nesting
							expression_builder.push('(');
						}

						expression_builder.push(char::from_digit(number_permutation[i] as u32, 10).unwrap());

						if i == rparen_pos {
							expression_builder.push(')');
						}


						if i != input_len - 1 {
							expression_builder.push(operator_permutator.get_operator_at(i));
						}
					}


					// h
					let result = evaluator::evaluate(expression_builder.clone());

					if result == 10.0 {
						output.push(expression_builder);

						if find_all_solutions == false {
							return output;
						}
					}


					operator_permutator.increment();

					if operator_permutator.is_maxed == true {
						break 'operation_permutations;
					}
				}
			}


			parentheses_permutator.increment();

			if parentheses_permutator.is_maxed == true {
				break 'parentheses_permutations;
			}
		}
	}



	return output;
}



fn generate_permutations(input: &mut Vec<u8>) -> Vec<Vec<u8>> {
	let input_len = input.len();

	let mut output: Vec<Vec<u8>> = vec![];
	let mut state: Vec<usize> = vec![0; input_len];


	output.push(input.clone());

	let mut pointer = 1;

	// quite honestly i have no idea how this works i just ripped it from wikipedia (heap's algorithm)
	while pointer < input_len {
		if state[pointer] < pointer {
			if pointer % 2 == 0 {
				input.swap(0, pointer);
			} else {
				input.swap(state[pointer], pointer);
			}

			output.push(input.clone());

			state[pointer] += 1;

			pointer = 1;
		} else {
			state[pointer] = 0;
			pointer += 1;
		}
	}


	let mut set = HashSet::new();


	for i in 0..input.len() {
		if set.contains(&input[i]) {
			break;
		}

		set.insert(input[i]);
	}

	// if there are duplicates in the input, they will not be present in the set
	// or if broken early
	if set.len() != input_len {

		output.sort();
		output.dedup();
	}


	return output;
}



#[cfg(test)]
#[test]
fn test_brute_forcer() {
	let config_1 = Config {
		input_digits: vec![8, 2, 7, 1],
		enabled_operations: String::from("+-*/"),

		find_all_solutions: false,
		solve_with_parentheses: false
	};


	let computation_1 = brute_force(config_1);
	assert_eq!(evaluator::evaluate(computation_1[0].clone()), 10.0);


	let config_2 = Config {
		input_digits: vec![5, 1, 6, 3],
		enabled_operations: String::from("+-*/"),

		find_all_solutions: false,
		solve_with_parentheses: false
	};

	let computation_2 = brute_force(config_2);
	assert_eq!(evaluator::evaluate(computation_2[0].clone()), 10.0);



	// with parentheses

	let config_3 = Config {
		input_digits: vec![9, 9, 1, 1],
		enabled_operations: String::from("+-*/"),

		find_all_solutions: false,
		solve_with_parentheses: true
	};

	let computation_3 = brute_force(config_3);
	assert_eq!(evaluator::evaluate(computation_3[0].clone()), 10.0);


	let config_4 = Config {
		input_digits: vec![5, 1, 1, 1],
		enabled_operations: String::from("+-*/"),

		find_all_solutions: false,
		solve_with_parentheses: true
	};

	let computation_4 = brute_force(config_4);
	assert_eq!(evaluator::evaluate(computation_4[0].clone()), 10.0);
}
