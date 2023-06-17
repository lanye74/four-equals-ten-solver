use std::process;

mod profile;
use profile::is_using_profile;

use four_equals_ten::{Configurator, run};



fn main() {
	let mut configurator = Configurator::new();

	let config = configurator.build_config().unwrap_or_else(|err| {
		eprintln!("Error building config: {err}");
		process::exit(1);
	});


	// don't print if debugging via flamegraph
	let print_solutions = !is_using_profile("flamegraph");


	run(&config, print_solutions);
}
