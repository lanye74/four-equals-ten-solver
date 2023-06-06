pub struct ParenthesesPermutator {
	lparen_pos: usize,
	rparen_pos: usize,

	input_length: usize,
	is_maxed: bool
}



impl ParenthesesPermutator {
	pub fn new(input_length: usize) -> ParenthesesPermutator {
		return ParenthesesPermutator {
			lparen_pos: 0,
			rparen_pos: 1,

			input_length,
			is_maxed: false
		};
	}

	// generates (0, 1) (0, 2) (1, 2) (0, 3) (1, 3) (2, 3) for n = 4
	fn increment(&mut self) {
		self.lparen_pos += 1;

		// making this condition > would generate (0, 0) (0, 1) (1, 0) (1, 1) ... (3, 3); those are all valid solutions too
		if self.lparen_pos == self.rparen_pos {
			self.lparen_pos = 0;
			self.rparen_pos += 1;

			if self.rparen_pos == self.input_length {
				self.is_maxed = true;

				return;
			}
		}


		// for n = 4, this condition triggers on (0, 3)
		if self.lparen_pos == 0 && (self.rparen_pos == (self.input_length - 1)) {
			// skip it. it's redundant, even if it is a valid solution
			self.increment();
		}
	}

	fn get_state(&self) -> (usize, usize) {
		return (self.lparen_pos, self.rparen_pos);
	}
}



impl Iterator for ParenthesesPermutator {
	type Item = (usize, usize);

	fn next(&mut self) -> Option<Self::Item> {
		if self.is_maxed {
			return None;
		}

		let output = self.get_state();

		self.increment();

		return Some(output);
	}
}
