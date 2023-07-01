pub struct OperatorPermutator<'a> {
	nodes: Vec<usize>,
	nodes_length: usize,
	is_maxed: bool,

	operator_mapper: &'a OperatorMapper,
	unique_operators: usize
}



impl OperatorPermutator<'_> {
	pub fn new(operator_mapper: &OperatorMapper, num_nodes: usize) -> OperatorPermutator {
		return OperatorPermutator {
			nodes_length: num_nodes,
			nodes: vec![0; num_nodes],

			unique_operators: operator_mapper.map.len(),
			operator_mapper,

			is_maxed: false
		};
	}

	fn increment(&mut self) {
		self.nodes[0] += 1;

		// wrap values
		for i in 0..self.nodes_length {
			// operator is above max value, wrap it
			if self.nodes[i] == self.unique_operators  {
				self.nodes[i] = 0;

				if i + 1 == self.nodes_length {
					// attempting to wrap the last node
					self.is_maxed = true;

					break;
				}

				self.nodes[i + 1] += 1;
			}
		}
	}

	fn state_as_char_vec(&mut self) -> Vec<char> {
		// TODO: use collect_into when it becomes stable
		return self.nodes.iter()
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
	// TODO: use Arc here? refer to video i watcjed
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
