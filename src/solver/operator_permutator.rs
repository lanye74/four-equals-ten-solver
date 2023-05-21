use std::collections::HashMap;



pub struct OperatorPermutator<'a> {
	state: Vec<usize>,
	state_length: usize,
	is_maxed: bool,

	operator_mapper: &'a OperatorMapper,
	unique_operators: usize
}



impl OperatorPermutator<'_> {
	pub fn new(operator_mapper: &OperatorMapper, num_nodes: usize) -> OperatorPermutator {
		// let operator_mapper = OperatorMapper::new(&enabled_operations);

		return OperatorPermutator {
			state_length: num_nodes,
			state: vec![0; num_nodes],

			unique_operators: operator_mapper.len,
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



impl Iterator for OperatorPermutator<'_> {
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



pub struct OperatorMapper {
	map: HashMap<usize, char>,
	len: usize
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

		return OperatorMapper {
			len: map.len(),
			map
		};
	}

	pub fn map(&self, i: usize) -> &char {
		// char implements copy trait. no need to clone
		return self.map.get(&i).unwrap();
	}
}
