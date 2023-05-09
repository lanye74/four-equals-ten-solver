use crate::solver::evaluator;
use super::OperatorPermutator;
use super::ParenthesesPermutator;



pub fn brute_force(mut input: Vec<u8>, available_operations: String, find_all_solutions: bool, solve_with_parentheses: bool) -> Vec<String> {
	let input_len = input.len();

	let number_permutations = generate_permutations(&mut input);

	let mut operator_permutator = OperatorPermutator::new(available_operations, input_len - 1);

	let mut output = vec![];


	// attempt to solve without parentheses
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
					expression_builder.push(*operator_permutator.get_operator_at(i));
				}
			}

			let result = evaluator::evaluate(expression_builder.clone());

			if result == 10f32 {
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
							expression_builder.push(*operator_permutator.get_operator_at(i));
						}
					}


					// h
					let result = evaluator::evaluate(expression_builder.clone());

					if result == 10f32 {
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
	let len = input.len();

	let mut output: Vec<Vec<u8>> = vec![];
	let mut state: Vec<usize> = vec![0; len];


	output.push(input.clone());

	let mut i = 1;

	// quite honestly i have no idea how this works i just ripped it from wikipedia (heap's algorithm)
	while i < len {
		if state[i] < i {
			if i % 2 == 0 {
				input.swap(0, i);
			} else {
				input.swap(state[i], i);
			}

			output.push(input.clone());

			state[i] += 1;

			i = 1;
		} else {
			state[i] = 0;
			i += 1;
		}
	}

	// TODO: check for duplicates in the input before doing this
	output.sort();
	output.dedup();

	return output;
}



#[cfg(test)]
#[test]
fn test_brute_forcer() {
	let computation_1 = brute_force(vec![8, 2, 7, 1], String::from("+-*/"), false, false);
	assert_eq!(evaluator::evaluate(computation_1[0].clone()), 10f32);

	let computation_2 = brute_force(vec![5, 1, 6, 3], String::from("+-*/"), false, false);
	assert_eq!(evaluator::evaluate(computation_2[0].clone()), 10f32);

	// with parentheses
	let computation_3 = brute_force(vec![9, 9, 1, 1], String::from("+-*/"), false, true);
	assert_eq!(evaluator::evaluate(computation_3[0].clone()), 10f32);

	let computation_4 = brute_force(vec![5, 1, 1, 1], String::from("+-*/"), false, true);
	assert_eq!(evaluator::evaluate(computation_4[0].clone()), 10f32);
}
