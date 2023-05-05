#[derive(PartialEq)]
#[derive(Clone)]
// #[derive(Debug)]
pub enum Token {
	Add,
	Subtract,
	Multiply,
	Divide,
	LParen,
	RParen,

	Number(f32)
}



pub fn tokenize(input: String) -> Vec<Token> {
	// this function assumes that any numbers input are positive single-digit integers, which is true for 4=10
	// if i was writing a tokenizer for more complex inputs, we might have issues
	// such is not the case : )
	let characters = input.chars();

	let mut output: Vec<Token> = Vec::new();

	for (_index, character) in characters.enumerate() {
		let token = map_to_token(character);
		output.push(token);
	}

	return output;
}



pub fn map_to_token(character: char) -> Token {
	return match character {
		'+' => Token::Add,
		'-' => Token::Subtract,
		'*' => Token::Multiply,
		'/' => Token::Divide,
		'(' => Token::LParen,
		')' => Token::RParen,

		other => {
			// return Token::Number((other.to_digit() as f32)
			return Token::Number(char::to_digit(other, 10)
				.expect("token mapper received invalid input!") as f32
			);
		}
	};
}



#[cfg(test)]
#[test]
fn test_tokenizer() {
	use crate::util;

	let result = tokenize(String::from("(3*5)/7+0-1*2*9/(8+4)*6"));

	let expected = vec![
		Token::LParen, Token::Number(3.0), Token::Multiply, Token::Number(5.0), Token::RParen,
		Token::Divide, Token::Number(7.0), Token::Add, Token::Number(0.0), Token::Subtract, Token::Number(1.0),
		Token::Multiply, Token::Number(2.0), Token::Multiply, Token::Number(9.0), Token::Divide,
		Token::LParen, Token::Number(8.0), Token::Add, Token::Number(4.0), Token::RParen,
		Token::Multiply, Token::Number(6.0)
	];


	let are_equal = util::vecs_are_equal(result, expected);

	if are_equal == false {
		panic!("tokenization does not match expected!");
	}
}
