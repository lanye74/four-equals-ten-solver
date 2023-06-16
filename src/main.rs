mod profile;
use profile::is_using_profile;

use four_equals_ten::{Configurator, run};



fn main() {
	let mut configurator = Configurator::new();

	// don't print if debugging via flamegraph
	let print_solutions = !is_using_profile("flamegraph");


	let config = configurator.build_config();

	run(&config, print_solutions);
}
