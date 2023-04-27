use crate::evaluator::evaluator;



pub fn brute_force(input: &mut Vec<u8>) -> String {
	evaluator::evaluate(String::from("6*(1/9*9+2)+3/2"));

	let permutations = generate_permutations(input);

	dbg!(&permutations);
	dbg!(permutations.len());

	// attempt to solve without parentheses
	for permutation in permutations.iter() {
		let mut operator_state: Vec<u8> = vec![0; input.len() - 1];

		let mut expression = String::new();

		for i in 0..(input.len()) {
			expression.push(char::from_digit(permutation[i] as u32, 10).unwrap());

			if i != input.len() - 1 {
				expression.push(map_number_to_operator(operator_state[i]));
			}
		}


		operator_state[0] += 1;

		for i in 0..(operator_state.len()) {
			let operator = operator_state[i];

			if operator == 4 {
				operator_state[i] = 0;

				if i + 1 == operator_state.len() {
					// done iterating
				}

				operator_state[i + 1] += 1;
			}
		}
	}

	return String::from("");
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

	// quite honestly i have no idea how this works i juts ripped it from wikipedia
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



// fn factorial(of: usize) -> usize {
// 	let mut product: usize = 1;

// 	for i in 2..=of {
// 		product *= i;
// 	}

// 	return product;
// }
