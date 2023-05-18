use std::collections::HashMap;



pub struct OperatorPermutator {
	state: Vec<usize>,
	state_length: usize,
	is_maxed: bool,

	operator_mapper: OperatorMapper,
	unique_operators: usize
}



impl OperatorPermutator {
	pub fn new(enabled_operations: &String, num_nodes: usize) -> OperatorPermutator {
		let operator_mapper = OperatorMapper::new(&enabled_operations);

		return OperatorPermutator {
			state_length: num_nodes,
			state: vec![0; num_nodes],

			unique_operators: operator_mapper.len(),
			operator_mapper,

			is_maxed: false
		};
	}

	fn increment(&mut self) {
		self.state[0] += 1;

		// wrap values
		for i in 0..self.state_length {
			// operator is above max value, wrap it
			if self.state[i] == self.unique_operators  {
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

	fn state_as_char_vec(&mut self) -> Vec<char> {
		let mut output = vec![];

		for element in self.state.iter() {
			output.push(*self.operator_mapper.map(*element));
		}

		return output;
	}
}



impl Iterator for OperatorPermutator {
	type Item = Vec<char>;

	fn next(&mut self) -> Option<Self::Item> {
		let output = self.state_as_char_vec();

		if self.is_maxed == false {
			self.increment();

			return Some(output);
		}

		return None;
	}
}



struct OperatorMapper {
	map: HashMap<usize, char>
}



impl OperatorMapper {
	pub fn new(enabled_operations: &String) -> OperatorMapper {
		let operations = enabled_operations
			.chars()
			.enumerate();

		let mut map: HashMap<usize, char> = HashMap::new();

		for (i, operation) in operations {
			map.insert(i, operation);
		}

		return OperatorMapper {map};
	}

	pub fn map(&self, i: usize) -> &char {
		// char implements copy trait. no need to clone
		return self.map.get(&i).unwrap();
	}

	pub fn len(&self) -> usize {
		return self.map.len();
	}
}
