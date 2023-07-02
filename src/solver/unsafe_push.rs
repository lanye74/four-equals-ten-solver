pub trait UnsafePush {
	fn unsafe_push(&mut self, char: char);
}



impl UnsafePush for String {
	// basically i just dug into String::push and ctrl + c ctrl + v without much thinking. skipping a few checks makes the pushes faster
	// e.g. the char is always 1 byte, the string always has enough space, etc.

	// ideally i would avoid future dirty implementations like this butttttttt it makes number go down
	fn unsafe_push(&mut self, char: char) {
		unsafe {
			let len = self.len();
			let self_as_vec = self.as_mut_vec();

			let end = self_as_vec.as_mut_ptr().add(len);
			std::ptr::write(end, char as u8);
			self_as_vec.set_len(len + 1);
		}
	}
}
