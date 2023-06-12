mod io_reader;

mod profile;
use profile::is_using_profile;

mod configurator;
use configurator::Configurator;

mod solver;



fn main() {
	// TODO: cleanup main
	let mut configurator = Configurator::new();

	// don't print if debugging via flamegraph
	let print_solutions = !is_using_profile("flamegraph");


	let config = configurator.build_config();

	let output = solver::brute_force(&config);


	if output.solutions.is_empty() {
		println!("No solutions found!");
		println!("Time taken: {:?}", output.time_taken);
		println!("Solutions considered: {}", output.solutions_considered);

		return;
	}


	let solutions_len = output.solutions.len();

	println!("Solution{} found!:", if solutions_len > 1 {"s"} else {""});

	// if print_solutions == true {
	// 	for sol in output.solutions {
	// 		println!("{}", sol);
	// 	}
	// }


	if solutions_len > 1 {
		println!("Total: {}", solutions_len);
	}


	println!("Time taken: {:?}", output.time_taken);
	println!("Solutions considered: {}", output.solutions_considered);
}
