pub fn get_cargo_profile() -> Option<String> {
	return Some(std::env::args()
		.nth(0)? // exe path
		.split(std::path::MAIN_SEPARATOR) // split into its components
		.nth_back(1)? // isolate the directory of the exe
		.to_owned());
}
