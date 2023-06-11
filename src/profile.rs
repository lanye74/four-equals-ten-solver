pub fn is_using_profile(profile_name: &'static str) -> bool {
	let exe_path = std::env::args().nth(0);

	// if let is still kinda hard for me to read so, unwrap exe_path if it is Some
	if let Some(exe_path) = exe_path {
		let exe_dir = exe_path.split(std::path::MAIN_SEPARATOR).nth_back(1);

		// return false if None, else return the value of the closure
		return exe_dir.is_some_and(|dir| dir == profile_name);
	}


	return false;
}
