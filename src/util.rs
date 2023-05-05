use crate::solver::Token;



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
	for index in 0..(vec.len()) {
		let element = &vec[index];

		if *element == token {
			return index;
		}
	}

	return usize::MAX;
}



pub fn find_token_in_range(vec: &Vec<Token>, token: Token, lower_bound: usize, upper_bound: usize) -> usize {
	for index in lower_bound..=upper_bound {
		let element = &vec[index];

		if *element == token {
			return index;
		}
	}

	return usize::MAX;
}



pub fn unwrap_token(number: &Token) -> f32 {
	return match number {
		Token::Number(value) => *value,
		_ => panic!("unwrap_token called with non-number!")
	};
}
