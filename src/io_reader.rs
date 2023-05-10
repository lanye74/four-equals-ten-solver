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
	pub fn read_line(&self, prompt: &'static str) -> String {
		let mut buffer = String::new();

		println!("{}", prompt);

		self.stdin
			.read_line(&mut buffer)
			.expect("should've been able to read line!");

		return buffer;

	}
}



// extension methods of read and read_line
impl IOReader {
	pub fn yn_prompt(&mut self, prompt: &'static str) -> bool {
		let result = self.read(prompt);

		return match result.trim().to_ascii_lowercase().as_str() {
			"y" => true,
			"n" => false,
			_ => panic!("Invalid input!")
		};
	}
}
