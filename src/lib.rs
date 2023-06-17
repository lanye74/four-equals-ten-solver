mod configurator;

pub use configurator::{Config, Configurator};

mod io_reader;

mod solver;



// TODO: make print_solutions part of Config
pub fn run(config: &Config, print_solutions: bool) {
	let output = solver::brute_force(config);

	if output.solutions.is_empty() {
		println!("No solutions found!");
		println!("Time taken: {:?}", output.time_taken);
		println!("Solutions considered: {}", output.solutions_considered);
	}


	let solutions_len = output.solutions.len();

	println!("Solution{} found!:", if solutions_len > 1 {"s"} else {""});

	if print_solutions == true {
		for sol in output.solutions {
			println!("{}", sol);
		}
	}


	if solutions_len > 1 {
		println!("Total: {}", solutions_len);
	}


	println!("Time taken: {:?}", output.time_taken);
	println!("Solutions considered: {}", output.solutions_considered);
}
