mod io_reader;

mod configurator;
use configurator::Configurator;

mod solver;



fn main() {
	let mut configurator = Configurator::new();

	let config = configurator.build_config();

	let output = solver::brute_force(&config);


	if output.solutions.is_empty() {
		println!("No solutions found!");
	} else {
		let solutions_len = output.solutions.len();


		println!("Solution{} found!:",
			if solutions_len > 1 {"s"} else {""}
		);

		// for sol in output.solutions {
		// 	println!("{}", sol);
		// }

		if config.find_all_solutions == true {
			println!("Total: {}", solutions_len);
		}
	}


	println!("Time taken: {:?}", output.time_taken);
	println!("Solutions considered: {}", output.solutions_considered);
}
