use crate::evaluator::evaluator;



pub fn brute_force(input: &mut Vec<u8>) -> String {
	let permutations = generate_permutations(input);

	let input_len = input.len();

	let mut output = String::new();

	// attempt to solve without parentheses
	'permutation_loop: for permutation in permutations.iter() {
		let mut operator_state: Vec<u8> = vec![0; input_len - 1];

		let operator_state_len = operator_state.len();


		'operation_loop: loop {
			let mut expression = String::new();


			for i in 0..input_len {
				expression.push(char::from_digit(permutation[i] as u32, 10).unwrap());

				if i != input_len - 1 {
					expression.push(map_number_to_operator(operator_state[i]));
				}
			}

			let result = evaluator::evaluate(expression.clone());

			if result == 10.0 {
				// winner found!
				output = expression;
				break 'permutation_loop;
			}


			// go to next operator state
			operator_state[0] += 1;

			for i in 0..operator_state_len {
				let operator = operator_state[i];

				// wrap operator states (s[0] = 4; s[1] = 0; --> s[0] = 0; s[1] = 1);
				if operator == 4 {
					operator_state[i] = 0;

					if i + 1 == operator_state_len {
						// worked through every operator combination; run the loop again
						// this is equivalent to break 'permutation loop but just for clarity
						break 'operation_loop;
					}

					operator_state[i + 1] += 1;
				}
			}
		}
	}


	return output;
}



fn map_number_to_operator(number: u8) -> char {
	return match number {
		0 => char::from('+'),
		1 => char::from('-'),
		2 => char::from('*'),
		3 => char::from('/'),

		_ => panic!("invalid number supplied to map_number_to_operator!")
	};
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

	output.sort();
	output.dedup();

	return output;
}



#[cfg(test)]
#[test]
fn test_brute_forcer() {
	assert_eq!(brute_force(&mut vec![8, 2, 7, 1]), String::from("8*2-7+1"));
	// this one has one other immediate solution i can think of, 5*3-6+1, but this is what it will output
	assert_eq!(brute_force(&mut vec![5, 1, 6, 3]), String::from("6*5/3*1"));

}
