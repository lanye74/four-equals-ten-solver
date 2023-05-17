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


		// for n = 4, this condition triggers on (0, 3)
		if self.lparen_pos == 0 && (self.rparen_pos == (self.input_length - 1)) {
			self.increment();
		}


		// i could also have this generate (0, 0) (0, 1) (1, 0) (1, 1) ... (3, 3)... because those are all valid solutions too

		// self.lparen_pos += 1;

		// if self.lparen_pos > self.rparen_pos {
		// 	self.lparen_pos = 0;
		// 	self.rparen_pos += 1;

		// 	if self.rparen_pos == self.input_length {
		// 		self.is_maxed = true;
		// 	}
		// }
	}

	pub fn get_state(&self) -> (usize, usize) {
		return (self.lparen_pos, self.rparen_pos);
	}

	#[allow(dead_code)]
	pub fn reset(&mut self) {
		self.lparen_pos = 0;
		self.rparen_pos = 1;
	}
}
