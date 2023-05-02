use std::io::{self, Write};



pub struct IOReader {
	stdin: io::Stdin,
	stdout: io::Stdout
}



impl IOReader {
	pub fn new() -> IOReader {
		return IOReader {
			stdin: io::stdin(),
			stdout: io::stdout()
		};
	}

	pub fn read(&mut self, prompt: &'static str) -> String {
		let mut buffer = String::new();

		print!("{}", prompt);
		self.stdout.flush()
			.expect("should've been able to flush buffer!");

		self.stdin
			.read_line(&mut buffer)
			.expect("should've been able to read line!");

		return buffer;
	}

	#[allow(dead_code)]
	fn read_line(&self, prompt: &'static str) -> String {
		let mut buffer = String::new();

		println!("{}", prompt);

		self.stdin
			.read_line(&mut buffer)
			.expect("should've been able to read line!");

		return buffer;

	}
}
