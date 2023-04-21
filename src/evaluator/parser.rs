use super::tokenizer::Token;



pub fn parse(mut input: Vec<Token>) -> f32 {
	// this function assumes there is only one set of parentheses, and that the input is valid
	// i might write an input validator later. but for now

	let mut input_len = input.len();


	let lparen_position = input.iter()
		.position(|token| *token == Token::LParen)
		.unwrap_or(255);


	if lparen_position != 255 {
		let rparen_position = input.iter()
			.position(|token| *token == Token::RParen)
			.unwrap(); // yes, this panics if not found -- but we're assuming input is valid


		// number of expressions to evaluate inside the parentheses
		// position of tokens in parentheses = (lparen + 1, rparen - 1)
		// divide by 2 because yes idk how to articulate it it works
		let num_expressions = (rparen_position - lparen_position - 2) / 2;

		let paren_value;

		if num_expressions == 1 {
			paren_value = evaluate_expression(&input[lparen_position + 1], &input[lparen_position + 2], &input[rparen_position - 1]);

			let mut input_new: Vec<Token> = vec![];

			let before_paren = &input[0..(lparen_position)];
			let after_paren = &input[(rparen_position + 1)..input_len];

			input_new.extend_from_slice(before_paren);
			input_new.push(Token::Number(paren_value));
			input_new.extend_from_slice(after_paren);

			input = input_new;
			input_len = input.len();
		}

		if num_expressions == 2 {

		}
	}


	return 3f32;
}



fn evaluate_expression(operand_one: &Token, operation: &Token, operand_two: &Token) -> f32 {
	// let operand_one_parsed = tokenizer::map_from_token(operand_one).parse::<f32>().unwrap();
	let operand_one = value_from_token(operand_one);
	let operand_two = value_from_token(operand_two);
	// let operand_two_parsed = tokenizer::map_from_token(operand_two).parse::<f32>().unwrap();

	return match operation {
		Token::Add => operand_one + operand_two,
		Token::Subtract => operand_one - operand_two,
		Token::Multiply => operand_one * operand_two,
		Token::Divide => operand_one / operand_two,

		_ => {
			panic!("invalid operation supplied to evaluate_expression!");
		}
	};
}



fn value_from_token(number: &Token) -> f32 {
	return match number {
		Token::Number(value) => *value,
		_ => panic!("value_from_token called with non-number!")
	};
}
