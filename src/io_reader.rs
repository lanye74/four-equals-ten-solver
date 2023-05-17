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

		return buffer.trim().to_owned();
	}

	#[allow(dead_code)]
	pub fn read_line(&self, prompt: &'static str) -> String {
		let mut buffer = String::new();

		println!("{}", prompt);

		self.stdin
			.read_line(&mut buffer)
			.expect("should've been able to read line!");

		return buffer.trim().to_owned();
	}

	pub fn read_with_default(&mut self, prompt: &'static str, default: String) -> String {
		let result = self.read(prompt);

		if result.is_empty() {
			return default;
		}

		return result;
	}
}



// extension methods of read and read_line
impl IOReader {
	pub fn yn_prompt(&mut self, prompt: &'static str) -> bool {
		let result = self.read(prompt);

		return match result.to_ascii_lowercase().as_str() {
			"y" => true,
			"n" => false,
			_ => panic!("Invalid input!")
		};
	}

	pub fn read_float_with_default(&mut self, prompt: &'static str, default: f32) -> f32 {
		let result = self.read_with_default(prompt, default.to_string());

		return match result.parse::<f32>() {
			Ok(value) => value,
			Err(..) => default
		};
	}
}
