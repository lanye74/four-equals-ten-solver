use std::time::{Instant};

mod io_reader;

mod configurator;
use configurator::Configurator;

mod solver;



fn main() {
	let mut configurator = Configurator::new();

	let config = configurator.build_config();
	let config_clone = config.clone();


	let start = Instant::now();
	let solutions = solver::brute_force(config);
	let elapsed = start.elapsed();


	if solutions.is_empty() {
		println!("No solutions found!");
	} else {
		let solutions_len = solutions.len();


		println!("Solution{} found!:",
			if solutions_len > 1 {"s"} else {""}
		);

		for sol in solutions {
			println!("{}", sol);
		}

		if config_clone.find_all_solutions == true {
			println!("Total: {}", solutions_len);
		}
	}


	println!("Time taken: {:?}", elapsed);

}
