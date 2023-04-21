use crate::util::find_token_from_position;
use super::tokenizer::Token;



pub fn parse(mut input: Vec<Token>) -> f32 {
	// this function assumes there is only one set of parentheses, and that the input is valid
	// i might write an input validator later. but for now

	let mut input_len = input.len();


	let lparen_position = input.iter()
		.position(|token| *token == Token::LParen)
		.unwrap_or(usize::MAX);


	if lparen_position != usize::MAX {
		let rparen_position = find_token_from_position(&input, Token::RParen, 0);


		// number of expressions to evaluate inside the parentheses
		// position of tokens in parentheses = (lparen + 1, rparen - 1)
		// divide by 2 because yes idk how to articulate it it works
		let mut num_expressions = (rparen_position - lparen_position - 2) / 2;


		while num_expressions > 0 {
			let multiply_pos = find_token_from_position(&input, Token::Multiply, lparen_position);
			let divide_pos = find_token_from_position(&input, Token::Divide, lparen_position);

			let mut operation_value = f32::MIN;

			let mut operation_pos = std::cmp::min(multiply_pos, divide_pos);

			if operation_pos != usize::MAX {
				operation_value = evaluate_expression(&input[operation_pos - 1], &input[operation_pos + 1], &input[operation_pos]);

				// i'm literally going insane. here just unwrap the value and insert it into input

				num_expressions -= 1;
				continue;
			}


			println!("{}", operation_value);
		}



		// let paren_value = 0f32;


		// let mut input_new: Vec<Token> = vec![];

		// let before_paren = &input[0..(lparen_position)];
		// let after_paren = &input[(rparen_position + 1)..input_len];

		// input_new.extend_from_slice(before_paren);
		// input_new.push(Token::Number(paren_value));
		// input_new.extend_from_slice(after_paren);

		// input = input_new;
		// input_len = input.len();
	}


	return 3f32;
}



fn evaluate_expression(operand_one: &Token, operand_two: &Token, operation: &Token) -> f32 {
	let operand_one = value_from_token(operand_one);
	let operand_two = value_from_token(operand_two);

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
