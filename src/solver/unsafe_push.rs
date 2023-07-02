pub trait UnsafePush {
	fn unsafe_push(&mut self, char: char);
	fn unsafe_push_digit(&mut self, digit: u8);
}



impl UnsafePush for String {
	// basically i just dug into String::push and ctrl + c ctrl + v without much thinking. skipping a few checks makes the pushes faster
	// e.g. the char is always 1 byte, the string always has enough space, etc.

	// ideally i would avoid future dirty implementations like this butttttttt it makes number go down
	fn unsafe_push(&mut self, char: char) {
		let char = char as u8;
		let len = self.len();

		unsafe {
			let self_as_vec = self.as_mut_vec();
			let end = self_as_vec.as_mut_ptr().add(len);
			//  std::ptr::write unwrapped (skipping checks that i can already verify)
			std::ptr::copy_nonoverlapping(&char as *const u8, end, 1);
			// skipped the forget because uhhhh i hate memory safety or smthn
			// the example of copy_nonoverlapping happens to be an implementation of vec::push and it doesn't forget sooo

			self_as_vec.set_len(len + 1);
		}
	}

	// convenience method to skip char::from_digit().unwrap()
	fn unsafe_push_digit(&mut self, digit: u8) {
		// 48 = b'0'
		self.unsafe_push((48 + digit) as char);
	}
}
