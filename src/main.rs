use std::{env, process};

use four_equals_ten::{Configurator, run};



fn main() {
	let mut configurator = Configurator::new();

	let force_no_print = env::args().any(|flag| flag == "--silent-output");

	let config = configurator.build_config(force_no_print).unwrap_or_else(|err| {
		eprintln!("Error building config: {err}");
		process::exit(1);
	});

	// let rapid_mode = env::args().nth(1).is_some_and(|flag| flag == "--rapid");

	run(&config);
}
