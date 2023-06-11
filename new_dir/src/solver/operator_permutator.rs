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

			unique_operators: operator_mapper.map.len(),
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
		// TODO: use collect_into when it becomes stable
		return self.state.iter()
			.map(|&operator| self.operator_mapper.map(operator))
			.collect();
	}
}



impl Iterator for OperatorPermutator<'_> {
	type Item = Vec<char>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.is_maxed == true {
			return None;
		}

		let output = self.state_as_char_vec();

		self.increment();

		return Some(output);
	}
}



pub struct OperatorMapper {
	map: Vec<char>
}



impl OperatorMapper {
	pub fn new(enabled_operations: &String) -> OperatorMapper {
		let operations = enabled_operations
			.chars()
			.collect::<Vec<char>>();

		// fun fact: map was once a hashmap. why? god knows! i'm insane!
		return OperatorMapper {
			map: operations
		};
	}

	pub fn map(&self, i: usize) -> char {
		return self.map[i];
	}
}
