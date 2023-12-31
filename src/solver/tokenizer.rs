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



pub fn map_char_to_token(character: char) -> Token {
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

		digit_as_char => Token::Number(
				digit_as_char.to_digit(10)
				.expect("map_char_to_token received invalid input!") as f32
			)
	};
}



pub fn format_tokens(tokens: &Vec<Token>) -> String {
	return tokens.iter()
		.map(|token| match token {
			Token::Add => '+',
			Token::Subtract => '-',
			Token::Multiply => '*',
			Token::Divide => '/',
			Token::LParen => '(',
			Token::RParen => ')',

			Token::Number(num) => char::from_digit(*num as u32, 10).unwrap()
		})
		.collect::<String>();
}



#[cfg(test)]
#[test]
fn test_tokenizer() {
	fn vecs_are_equal<T: std::cmp::PartialEq>(vec1: &[T], vec2: &[T]) -> bool {
		if vec1.len() != vec2.len() {
			return false;
		}

		// god i love iterators
		return vec1.iter()
			.zip(vec2.iter())
			.all(|(el1, el2)| el1 == el2);
	}



	let result = String::from("(3*5)/7+0-1*2*9/(8+4)*6")
		.chars()
		.map(map_char_to_token)
		.collect::<Vec<Token>>();

	let expected = vec![
		Token::LParen, Token::Number(3.0), Token::Multiply, Token::Number(5.0), Token::RParen,
		Token::Divide, Token::Number(7.0), Token::Add, Token::Number(0.0), Token::Subtract, Token::Number(1.0),
		Token::Multiply, Token::Number(2.0), Token::Multiply, Token::Number(9.0), Token::Divide,
		Token::LParen, Token::Number(8.0), Token::Add, Token::Number(4.0), Token::RParen,
		Token::Multiply, Token::Number(6.0)
	];


	let are_equal = vecs_are_equal(&result, &expected);

	assert!(are_equal == true, "tokenization does not match expected!");
}



// to simplify my testing functions
#[cfg(test)]
pub fn tokenize(expression: &String) -> Vec<Token> {
	return expression
		.chars()
		.map(map_char_to_token)
		.collect();
}
