use crate::evaluator::Token;



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



pub fn find_token_from_position(vec: &Vec<Token>, token: Token, position: usize) -> usize {
	let mut index = position;

	while index < vec.len() {
		let element = &vec[index];

		if *element == token {
			return index;
		}

		index += 1;
	}

	return usize::MAX;
}



// fn find_token(input: &Vec<Token>, item: Token) -> usize {
// 	return input.iter()
// 		.position(|token| *token == item)
// 		.unwrap_or(usize::MAX);
// }
