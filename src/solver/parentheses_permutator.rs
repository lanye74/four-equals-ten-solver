pub struct ParenthesesPermutator {
	lparen_pos: usize,
	rparen_pos: usize,

	input_length: usize,
	pub is_maxed: bool
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
	pub fn increment(&mut self) {
		self.lparen_pos += 1;

		if self.lparen_pos == self.rparen_pos {
			self.lparen_pos = 0;
			self.rparen_pos += 1;

			if self.rparen_pos == self.input_length {
				self.is_maxed = true;
			}
		}

		// if self.lparen_pos == 0 && (self.rparen_pos == (self.input_length - 1)) {
		// 	// for n = 4, this is (0, 3)
		// 	self.increment();
		// }
	}

	pub fn get_state(&self) -> (usize, usize) {
		return (self.lparen_pos, self.rparen_pos);
	}
}
