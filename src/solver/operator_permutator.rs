use std::collections::HashMap;

// #[derive(Debug)]
pub struct OperatorPermutator {
	state: Vec<u8>,
	state_length: usize,
	pub is_maxed: bool,

	#[allow(dead_code)]
	operator_mapper: OperatorMapper,
	num_operators: usize
}



impl OperatorPermutator {
	pub fn new(enabled_operations: String, length: usize) -> OperatorPermutator {
		let operator_mapper = OperatorMapper::new(enabled_operations);

		return OperatorPermutator {
			state_length: length,
			state: vec![0; length],

			num_operators: operator_mapper.len(),
			operator_mapper,

			is_maxed: false
		};
	}

	// this boolean value returned indicates whether the state is "maxed" or not
	// that is to say, whether OperatorPermutator_instance.state == vec![3, 3, 3, 3];
	pub fn increment(&mut self) {
		self.state[0] += 1;

		self.wrap_values();
	}

	pub fn get_operator_at(&self, i: usize) -> &char {
		return self.operator_mapper.map(self.state[i]);
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
		// for i in 0..self.state_length {
		//	self.state[i] = 0;
		// }

		self.state = vec![0; self.state_length];

		self.is_maxed = false;
	}
}



struct OperatorMapper {
	map: HashMap<u8, char>
}



impl OperatorMapper {
	pub fn new(enabled_operations: String) -> OperatorMapper {
		let operations = enabled_operations
			.chars()
			.into_iter();

		let mut map: HashMap<u8, char> = HashMap::new();

		for (i, operation) in operations.enumerate() {
			map.insert(i as u8, operation);
		}

		return OperatorMapper {map};
	}

	pub fn map(&self, i: u8) -> &char {
		// char implements copy trait. no need to clone
		return self.map.get(&i).unwrap();
	}

	pub fn len(&self) -> usize {
		return self.map.len();
	}
}
