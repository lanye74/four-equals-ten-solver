pub fn vecs_are_equal<T: std::cmp::PartialEq>(vec1: Vec<T>, vec2: Vec<T>) -> bool {
	if vec1.len() != vec2.len() {
		return false;
	}


	let len = vec1.len();


	for i in 0..len {
		let is_equal = vec1[i] == vec2[i];

		if is_equal {
			continue;
		} else {
			return false;
		}
	}


	return true;
}
