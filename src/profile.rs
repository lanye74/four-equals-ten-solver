pub fn get_cargo_profile() -> String {
	// i feel like this could be simplified but o well
	return std::env::args()
		.nth(0) // exe path
		.unwrap_or_default()
		.split(std::path::MAIN_SEPARATOR) // split into its components
		.nth_back(1) // isolate the directory of the exe
		.unwrap_or_default()
		.to_owned();
}
