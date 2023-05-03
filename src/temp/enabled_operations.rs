#![allow(dead_code)]



#[derive(Clone)]
pub struct EnabledOperations {
	pub addition: bool,
	pub subtraction: bool,
	pub multiplication: bool,
	pub division: bool,
	pub parentheses: bool
}



impl EnabledOperations {
	pub fn num_enabled(&self) -> usize {
		return
			self.bool_to_usize(self.addition) +
			self.bool_to_usize(self.subtraction) +
			self.bool_to_usize(self.multiplication) +
			self.bool_to_usize(self.division) +
			self.bool_to_usize(self.parentheses) * 2;
	}

	fn bool_to_usize(&self, input: bool) -> usize {
		return match input {
			true => 1 as usize,
			false => 0 as usize
		};
	}
}
