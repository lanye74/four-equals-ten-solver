use super::tokenizer::{Token, self};



pub fn parse(input: Vec<Token>) -> f32 {
	// this function assumes there is only one set of parentheses, and that the input is valid
	// i might write an input validator later. but for now


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

		if num_expressions == 1 {
			let paren_value = evaluate_expression(&input[lparen_position + 1], &input[lparen_position + 2], &input[rparen_position - 1]);
		}
	}


	return 3f32;
}



fn evaluate_expression(operand_one: &Token, operation: &Token, operand_two: &Token) -> f32 {
	let operand_one_parsed = tokenizer::map_from_token(operand_one).parse::<f32>().unwrap();
	let operand_two_parsed = tokenizer::map_from_token(operand_two).parse::<f32>().unwrap();

	// let result = match operation {

	// };

	return 3f32;
}
