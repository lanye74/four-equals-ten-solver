use crate::io_reader::IOReader;



pub struct Configurator {
	io_reader: IOReader
}



impl Configurator {
	pub fn new() -> Configurator {
		return Configurator {
			io_reader: IOReader::new()
		};
	}

	pub fn build_config(&mut self) -> Config {
		let input_digits = self.get_input_digits();
		let allowed_operations = self.get_allowed_operations();

		let find_all_solutions = self.get_find_all_solutions();
		let use_parentheses = self.get_use_parentheses();

		return Config {
			input_digits,
			allowed_operations,

			find_all_solutions,
			use_parentheses
		};
	}

	fn get_input_digits(&mut self) -> Vec<u8> {
		let input_digits = self.io_reader.read("Enter your digits: ");

		let input_digits = input_digits
			.trim()
			.chars()
			.map(|char| char.to_digit(10).unwrap_or(255) as u8)
			.filter(|num| *num < 10)
			.collect::<Vec<u8>>();

		return input_digits;
	}

	fn get_allowed_operations(&mut self) -> String {
		let result = self.io_reader.read("Enter your available operations, not including parentheses: ");

		let result = result
			.trim()
			.chars()
			.filter(|char| *char == '+' || *char == '-' || *char == '*' || *char == '/')
			.into_iter()
			.collect::<String>();


		return result;
	}

	fn get_find_all_solutions(&mut self) -> bool {
		return self.io_reader.yn_prompt("Do you want to find all solutions? Y/N: ");
	}

	fn get_use_parentheses(&mut self) -> bool {
		return self.io_reader.yn_prompt("Do you want to find solutions with parentheses? Y/N: ");
	}
}



pub struct Config {
	pub input_digits: Vec<u8>,
	pub allowed_operations: String,

	pub find_all_solutions: bool,
	pub use_parentheses: bool
}
