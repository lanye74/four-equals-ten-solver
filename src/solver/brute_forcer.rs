use crate::solver::evaluator;
use super::OperatorPermutator;



pub fn brute_force(mut input: Vec<u8>, available_operations: String, find_all_solutions: bool, solve_with_parentheses: bool) -> Vec<String> {
	let input_len = input.len();

	let permutations = generate_permutations(&mut input);

	let mut operator_permutator = OperatorPermutator::new(available_operations, input_len - 1);

	let mut output = vec![];


	// attempt to solve without parentheses
	'permutation_loop: for permutation in permutations.iter() {
		operator_permutator.reset();

		'operation_loop: loop {
			let mut expression = String::new();

			// build expression
			for i in 0..input_len {
				expression.push(char::from_digit(permutation[i] as u32, 10).unwrap());

				// ensures that a dangling operator isn't placed
				if i != input_len - 1 {
					expression.push(operator_permutator.get_operator_at(i));
				}
			}

			let result = evaluator::evaluate(expression.clone());

			if result == 10f32 {
				// winner found!
				output.push(expression);

				if find_all_solutions == false {
					break 'permutation_loop;
				}
			}


			operator_permutator.increment();

			if operator_permutator.is_maxed == true {
				// worked through every operator combination; run the loop again
				// this is equivalent to continue 'permutation loop but just for clarity

				break 'operation_loop;
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

		// i need a struct to generate (0, 1) (0, 2) (1, 2) (0, 3) (1, 3) (2, 3) etc
		// 'parentheses_loop: loop {
		// 	'permutation_loop: for permutation in permutations.iter() {
		// 		'operation_loop: loop {
		// 			// kill me


		// 		}
		// 	}
		// }
	}




	return output;
}



struct ParenthesesPermutator {
	lparen_pos: usize,
	rparen_pos: usize,

	input_length: usize,
	pub is_maxed: bool
}



impl ParenthesesPermutator {
	pub fn new(input_length: usize) -> ParenthesesPermutator {
		return ParenthesesPermutator {
			lparen_pos: 0,
			rparen_pos: 1,

			input_length,
			is_maxed: false
		};
	}

	pub fn increment(&mut self) {
		self.lparen_pos += 1;

		if self.lparen_pos == self.rparen_pos {
			self.lparen_pos = 0;
			self.rparen_pos += 1;

			if self.rparen_pos == self.input_length {
				self.is_maxed = true;
			}
		}
	}
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
}
