#[derive(Clone, Debug, PartialEq)]
pub enum Token {
	Add,
	Subtract,
	Multiply,
	Divide,
	LParen,
	RParen,

	Number(f32)
}



// TODO: simplify this using collect_into when it hits stable (see https://github.com/rust-lang/rust/issues/94780)
pub fn tokenize(expression: &String) -> Vec<Token> {
	let mut output = Vec::with_capacity(expression.len());

	let mapped = expression
		.chars()
		.map(map_char_to_token);

	output.extend(mapped);

	return output;
}



fn map_char_to_token(character: char) -> Token {
	// this function assumes that any numbers input are positive single-digit integers, which is true for 4=10
	// if i was writing a tokenizer for more complex inputs, we might have issues
	// such is not the case : )

	return match character {
		'+' => Token::Add,
		'-' => Token::Subtract,
		'*' => Token::Multiply,
		'/' => Token::Divide,
		'(' => Token::LParen,
		')' => Token::RParen,

		digit_as_char => {
			return Token::Number(digit_as_char.to_digit(10)
				.expect("map_char_to_token received invalid input!") as f32
			);
		}
	};
}



#[cfg(test)]
#[test]
fn test_tokenizer() {
	fn vecs_are_equal<T: std::cmp::PartialEq>(vec1: Vec<T>, vec2: Vec<T>) -> bool {
		if vec1.len() != vec2.len() {
			return false;
		}

		// god i love iterators
		return vec1.into_iter()
			.zip(vec2.into_iter())
			.all(|(el1, el2)| el1 == el2);
	}


	let result = tokenize(&String::from("(3*5)/7+0-1*2*9/(8+4)*6"));

	let expected = vec![
		Token::LParen, Token::Number(3.0), Token::Multiply, Token::Number(5.0), Token::RParen,
		Token::Divide, Token::Number(7.0), Token::Add, Token::Number(0.0), Token::Subtract, Token::Number(1.0),
		Token::Multiply, Token::Number(2.0), Token::Multiply, Token::Number(9.0), Token::Divide,
		Token::LParen, Token::Number(8.0), Token::Add, Token::Number(4.0), Token::RParen,
		Token::Multiply, Token::Number(6.0)
	];


	let are_equal = vecs_are_equal(result, expected);

	assert!(are_equal == true, "tokenization does not match expected!");
}
