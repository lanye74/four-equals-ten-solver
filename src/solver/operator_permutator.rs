pub struct OperatorPermutator {
	pub state: Vec<u8>,
	// length of the state
	state_length: usize,
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
			operators
		};
	}

	// this boolean value returned indicates whether the state is "maxed" or not
	// that is to say, whether OperatorPermutator::new(String::from("+-*/"), 4).state == vec![3, 3, 3, 3];
	pub fn increment(&mut self) -> bool {
		self.state[0] += 1;

		return self.wrap_values();
	}

	fn wrap_values(&mut self) -> bool {
		for i in 0..self.state_length {
			let operator = self.state[i];

			if operator == self.num_operators as u8 {
				self.state[i] = 0;

				if i + 1 == self.state_length {
					// attempting to wrap the last node
					return true;
				}

				self.state[i + 1] += 1;
			}
		}

		return false;
	}
}
