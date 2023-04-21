#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
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

	let characters = input
		.split("")
		.filter(|char| *char != "");


	let mut token_output: Vec<Token> = Vec::new();

	for (_index, character) in characters.enumerate() {
		let token = map_to_token(character);
		token_output.push(token);
	}

	return token_output;
}



pub fn map_to_token(character: &str) -> Token {
	return match character {
		"+" => Token::Add,
		"-" => Token::Subtract,
		"*" => Token::Multiply,
		"/" => Token::Divide,
		"(" => Token::LParen,
		")" => Token::RParen,

		other => {
			return Token::Number(other.parse::<f32>().expect("token mapper received invalid input!"));
		}
	};
}



pub fn map_from_token(token: &Token) -> String {
	return match token {
		Token::Add => String::from("+"),
		Token::Subtract => String::from("-"),
		Token::Multiply => String::from("*"),
		Token::Divide => String::from("/"),

		other => {
			dbg!(other);
			panic!("numbers should be unwrapped!");
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


	let is_equal = util::vecs_are_equal(result, expected);

	if is_equal == false {
		panic!("tokenization does not match expected!");
	}
}
