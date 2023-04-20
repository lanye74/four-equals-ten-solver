#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token {
	Add,
	Subtract,
	Multiply,
	Divide,
	LParen,
	RParen,

	Zero,
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine
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

		"0" => Token::Zero,
		"1" => Token::One,
		"2" => Token::Two,
		"3" => Token::Three,
		"4" => Token::Four,
		"5" => Token::Five,
		"6" => Token::Six,
		"7" => Token::Seven,
		"8" => Token::Eight,
		"9" => Token::Nine,

		other => {
			dbg!(other);
			panic!("token mapper received invalid input!");
		}
	};
}



pub fn map_from_token(token: &Token) -> String {
	return match token {
		Token::Add => String::from("+"),
		Token::Subtract => String::from("-"),
		Token::Multiply => String::from("*"),
		Token::Divide => String::from("/"),

		Token::Zero => String::from("0"),
		Token::One => String::from("1"),
		Token::Two => String::from("2"),
		Token::Three => String::from("3"),
		Token::Four => String::from("4"),
		Token::Five => String::from("5"),
		Token::Six => String::from("6"),
		Token::Seven => String::from("7"),
		Token::Eight => String::from("8"),
		Token::Nine => String::from("9"),

		other => {
			dbg!(other);
			panic!("token mapper received invalid input!");
		}
	};
}



#[cfg(test)]
#[test]
fn test_tokenizer() {
	use crate::util;

	let result = tokenize(String::from("(3*5)/7+0-1*2*9/(8+4)*6"));

	let expected = vec![
		Token::LParen, Token::Three, Token::Multiply, Token::Five, Token::RParen,
		Token::Divide, Token::Seven, Token::Add, Token::Zero, Token::Subtract, Token::One,
		Token::Multiply, Token::Two, Token::Multiply, Token::Nine, Token::Divide,
		Token::LParen, Token::Eight, Token::Add, Token::Four, Token::RParen,
		Token::Multiply, Token::Six
	];


	let is_equal = util::vecs_are_equal(result, expected);

	if is_equal == false {
		panic!("tokenization does not match expected!");
	}
}
