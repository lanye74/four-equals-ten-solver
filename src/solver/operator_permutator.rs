// #[derive(Debug)]
pub struct OperatorPermutator {
	state: Vec<u8>,
	state_length: usize,
	pub is_maxed: bool,

	#[allow(dead_code)]
	operators: Vec<String>,
	num_operators: usize
}



impl OperatorPermutator {
	pub fn new(operators: String, length: usize) -> OperatorPermutator {
		let operators = operators
			.split("")
			.filter(|x| *x != "")
			.map(|x| x.to_owned())
			.collect::<Vec<String>>();

		return OperatorPermutator {
			state_length: length,
			state: vec![0; length],

			num_operators: operators.len(),
			operators,

			is_maxed: false
		};
	}

	// this boolean value returned indicates whether the state is "maxed" or not
	// that is to say, whether OperatorPermutator::new(String::from("+-*/"), 4).state == vec![3, 3, 3, 3];
	pub fn increment(&mut self) {
		self.state[0] += 1;

		self.wrap_values();
	}

	pub fn get_operator_at(&self, i: usize) -> char {
		return map_number_to_operator(self.state[i]);
	}

	fn wrap_values(&mut self) {
		for i in 0..self.state_length {
			let operator = self.state[i];

			// operator is above max value, wrap it

			if operator == self.num_operators as u8 {
				self.state[i] = 0;

				if i + 1 == self.state_length {
					// attempting to wrap the last node
					self.is_maxed = true;

					break;
				}

				self.state[i + 1] += 1;
			}
		}
	}

	pub fn reset(&mut self) {
		for i in 0..self.state_length {
			self.state[i] = 0;
		}

		self.is_maxed = false;
	}
}



// TODO: use hashmap
fn map_number_to_operator(number: u8) -> char {
	return match number {
		0 => char::from('+'),
		1 => char::from('-'),
		2 => char::from('*'),
		3 => char::from('/'),

		_ => panic!("invalid number supplied to map_number_to_operator!")
	};
}
