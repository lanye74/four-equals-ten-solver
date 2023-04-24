use crate::evaluator::Token;



#[cfg(test)]
pub fn vecs_are_equal<T: std::cmp::PartialEq>(vec1: Vec<T>, vec2: Vec<T>) -> bool {
	if vec1.len() != vec2.len() {
		return false;
	}

	let len = vec1.len();

	for i in 0..len {
		if vec1[i] != vec2[i] {
			return false;
		}
	}

	return true;
}



pub fn find_token(vec: &Vec<Token>, token: Token) -> usize {
	// println!("    find_token: searching for {:?}", token);
	for index in 0..(vec.len()) {
		let element = &vec[index];

		if *element == token {
			// println!("    find_token: found {:?} at position {}", token, index);
			return index;
		}
	}

	return usize::MAX;
}



pub fn find_token_in_range(vec: &Vec<Token>, token: Token, lower_bound: usize, upper_bound: usize) -> usize {
	// println!("find_token_i_r: searching for {:?} from range {} to {}", token, lower_bound, upper_bound);
	for index in lower_bound..=upper_bound {
		let element = &vec[index];

		if *element == token {
			// println!("find_token_i_r: found {:?} at position {}", token, index);
			return index;
		}
	}

	return usize::MAX;
}



// fn find_token(input: &Vec<Token>, item: Token) -> usize {
// 	return input.iter()
// 		.position(|token| *token == item)
// 		.unwrap_or(usize::MAX);
// }



// pub fn find_token_from_position(vec: &Vec<Token>, token: Token, position: usize) -> usize {
// 	for index in position..(vec.len()) {
// 		let element = &vec[index];

// 		if *element == token {
// 			return index;
// 		}
// 	}

// 	return usize::MAX;
// }
