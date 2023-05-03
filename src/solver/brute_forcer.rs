use crate::solver::evaluator;
use super::OperatorPermutator;



pub fn brute_force(mut input: Vec<u8>, available_operations: String) -> String {
	let input_len = input.len();

	let permutations = generate_permutations(&mut input);

	let mut operator_permutator = OperatorPermutator::new(available_operations, input_len - 1);

	let mut output = String::new();

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
				output = expression;
				break 'permutation_loop;
			}


			operator_permutator.increment();

			if operator_permutator.is_maxed == true {
				// worked through every operator combination; run the loop again
				// this is equivalent to continue 'permutation loop but just for clarity

				break 'operation_loop;
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

	// quite honestly i have no idea how this works i just ripped it from wikipedia
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
	let computation_1 = brute_force(vec![8, 2, 7, 1], String::from("+-*/"));
	assert_eq!(evaluator::evaluate(computation_1), 10f32);

	let computation_2 = brute_force(vec![5, 1, 6, 3], String::from("+-*/"));
	assert_eq!(evaluator::evaluate(computation_2), 10f32);
}
