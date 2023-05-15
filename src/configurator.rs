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
		let enabled_operations = self.get_enabled_operations();

		let target_number = self.get_target_number();

		let find_all_solutions = self.get_find_all_solutions();
		let solve_with_parentheses = self.get_solve_with_parentheses();

		return Config {
			input_digits,
			enabled_operations,

			target_number,

			find_all_solutions,
			solve_with_parentheses
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

	fn get_enabled_operations(&mut self) -> String {
		let result = self.io_reader.read("Enter your available operations, not including parentheses: ");

		let result = result
			.trim()
			.chars()
			.filter(|char| *char == '+' || *char == '-' || *char == '*' || *char == '/')
			.into_iter()
			.collect::<String>();


		return result;
	}

	fn get_target_number(&mut self) -> f32 {
		return self.io_reader.read_float_with_default("Enter your target number, or leave empty for 10: ", 10.0);
	}

	fn get_find_all_solutions(&mut self) -> bool {
		return self.io_reader.yn_prompt("Do you want to find all solutions? Y/N: ");
	}

	fn get_solve_with_parentheses(&mut self) -> bool {
		return self.io_reader.yn_prompt("Do you want to find solutions with parentheses? Y/N: ");
	}
}



#[derive(Clone)]
pub struct Config {
	pub input_digits: Vec<u8>,
	pub enabled_operations: String,

	pub target_number: f32,

	pub find_all_solutions: bool,
	pub solve_with_parentheses: bool
}
