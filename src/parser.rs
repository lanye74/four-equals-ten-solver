use crate::util;



#[derive(PartialEq)]
#[derive(Debug)]
enum Tokens {
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



pub fn parse(input: String) -> f32 {
	// ownership can be passed here. no need to ref
	let tokens = tokenize(input);

	dbg!(tokens);

	return 3f32;
}



fn tokenize(input: String) -> Vec<Tokens> {
	let characters = input
		.split("")
		.filter(|char| *char != "")
		.collect::<Vec<&str>>();


	// probably bad practice to .collect().iter() but yknow
	let mut token_output: Vec<Tokens> = Vec::new();

	for (_index, character) in characters.iter().enumerate() {
		// deref
		let character = *character;

		let token = match character {
			"+" => Tokens::Add,
			"-" => Tokens::Subtract,
			"*" => Tokens::Multiply,
			"/" => Tokens::Divide,
			"(" => Tokens::LParen,
			")" => Tokens::RParen,

			"0" => Tokens::Zero,
			"1" => Tokens::One,
			"2" => Tokens::Two,
			"3" => Tokens::Three,
			"4" => Tokens::Four,
			"5" => Tokens::Five,
			"6" => Tokens::Six,
			"7" => Tokens::Seven,
			"8" => Tokens::Eight,
			"9" => Tokens::Nine,

			other => {
				dbg!(other);
				panic!("token parser received invalid input!")
			}
		};

		token_output.push(token);
	}

	return token_output;
}



#[test]
fn test_tokenizer() {
	let result = tokenize(String::from("(3*5)/7+0-1*2*9/(8+4)*6"));

	let expected = vec![
		Tokens::LParen,
		Tokens::Three,
		Tokens::Multiply,
		Tokens::Five,
		Tokens::RParen,
		Tokens::Divide,
		Tokens::Seven,
		Tokens::Add,
		Tokens::Zero,
		Tokens::Subtract,
		Tokens::One,
		Tokens::Multiply,
		Tokens::Two,
		Tokens::Multiply,
		Tokens::Nine,
		Tokens::Divide,
		Tokens::LParen,
		Tokens::Eight,
		Tokens::Add,
		Tokens::Four,
		Tokens::RParen,
		Tokens::Multiply,
		Tokens::Six
	];


	let is_equal = util::vecs_are_equal(result, expected);

	if is_equal == false {
		panic!("tokenization does not match expected!");
	}
}
