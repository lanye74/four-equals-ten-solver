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

	pub fn read(&mut self, prompt: &'static str) -> Result<String, &'static str> {
		let mut buffer = String::new();

		print!("{}", prompt);
		if self.stdout.flush().is_err() {
			return Err("Unable to flush buffer!");
		}


		if self.stdin.read_line(&mut buffer).is_err() {
			return Err("Error reading line!");
		}

		return Ok(buffer.trim().to_owned());
	}

	#[allow(dead_code)]
	pub fn read_line(&self, prompt: &'static str) -> Result<String, &'static str> {
		let mut buffer = String::new();

		println!("{}", prompt);

		if self.stdin.read_line(&mut buffer).is_err() {
			return Err("Error reading line!");
		}

		return Ok(buffer.trim().to_owned());
	}

	pub fn read_with_default(&mut self, prompt: &'static str, default: String) -> String {
		// tbh if something goes wrong i probably should just panic because i do NOT feel like figuring this out
		// i don't really want to return a Result of a "read with default" because there should always be a Some/Ok value
		let result = self.read(prompt).ok().unwrap();

		if result.is_empty() {
			return default;
		}

		return result;
	}
}



// extension methods of read, read_line, and read_with_default
impl IOReader {
	pub fn yn_prompt(&mut self, prompt: &'static str, default: Option<bool>) -> Result<bool, &'static str> {
		let result = self.read(prompt)?;

		return match result.to_ascii_lowercase().as_str() {
			"y" => Ok(true),
			"n" => Ok(false),
			// Option --> Result
			_ => default.ok_or("Invalid input!")
		}
	}

	pub fn read_float_with_default(&mut self, prompt: &'static str, default: f32) -> f32 {
		let result = self.read_with_default(prompt, default.to_string());

		return result.parse::<f32>().unwrap_or(default);
	}
}
