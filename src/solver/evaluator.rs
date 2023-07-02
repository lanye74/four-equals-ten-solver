use super::tokenizer::Token;



pub fn evaluate_tokens(tokens: &mut Vec<Token>) -> f32 {
	// this function assumes there is only one set of parentheses, and that the input is valid
	// i might write an input validator later. but for now

	// search for parentheses
	let lparen_pos = find_token(&tokens, Token::LParen);

	// if there is a set of parentheses
	if lparen_pos != usize::MAX {
		let mut rparen_pos = find_token(&tokens[lparen_pos..], Token::RParen) + lparen_pos;

		// calculate number of expressions to evaluate inside the parentheses
		// position of tokens in parentheses = (lparen + 1, rparen - 1)
		// divide by 2 because yes idk how to articulate it it works
		let mut num_expressions = (rparen_pos - lparen_pos - 2) / 2;

		// loop over every expression
		while num_expressions > 0 {
			if num_expressions != 1 {
				// find where the next operator is (mult/div - add/sub)
				// TODO: possibly find all operator positions in the string, and use an iter to find next
				let operator_pos = find_next_operator_pos(&tokens[lparen_pos..=rparen_pos]) + lparen_pos;

				// compute the expression
				let operation_value = evaluate_expression(&tokens[(operator_pos - 1)..=(operator_pos + 1)]);

				// NOTE TO SELF: you cannot immediately return here if evaluate_expression is f32::INFINITY, because x/INF = 0
				// i verified this in 4=10; the game also believes that x/INF = 0
				// e.g. 2/(6/0)+7+9/3 = 10
				// if the game support 6 digit inputs, it would also say that equaled 10
				// in the base game, with 4 digit inputs, early returning on operation_value = f32::INFINITY would be a valid optimization
				// you need at least 5 digits to encounter this technicality
				// 1/(1/0)+9+1 = 10


				// replace [..., operand_one, operation, operand_two, ...] with [..., result, ...]
				substitute_expression(tokens, operator_pos, operation_value);
			} else {
				// this is the last operation so the location of the expression is already guaranteed
				let operation_value = evaluate_expression(&tokens[(lparen_pos + 1)..=(rparen_pos - 1)]);

				substitute_expression_and_remove_paren(tokens, lparen_pos, rparen_pos, operation_value);

				break;
			}

			// rparen has moved because of substitution (this will always be by 2, since substitute_expression replaces one element and removes two). update it
			rparen_pos -= 2;

			num_expressions -= 1;
		}
	}


	let input_len = tokens.len();

	let mut num_expressions = (input_len - 1) / 2;

	while num_expressions > 0 {
		if num_expressions != 1 {
			let operator_pos = find_next_operator_pos(&tokens);

			let operation_value = evaluate_expression(&tokens[(operator_pos - 1)..=(operator_pos + 1)]);

			substitute_expression(tokens, operator_pos, operation_value);
		} else {
			let operation_value = evaluate_expression(&tokens);

			return operation_value;
		}


		num_expressions -= 1;
	}

	// this will never be reached, but cargo throws a fit if i don't
	return unwrap_number_token(&tokens[0]);
}



fn find_next_operator_pos(input: &[Token]) -> usize {
	let mut add_pos = usize::MAX;
	let mut subtract_pos = usize::MAX;

	for (index, token) in input.iter().enumerate() {
		match token {
			Token::Multiply | Token::Divide => return index,

			Token::Add if add_pos == usize::MAX => add_pos = index,
			Token::Subtract if subtract_pos == usize::MAX => subtract_pos = index,

			_ => {}
		};
	}


	return std::cmp::min(add_pos, subtract_pos);
}



fn substitute_expression(input: &mut Vec<Token>, operator_pos: usize, value: f32) {
	// this works too, but is cringe and doesn't look nearly as cool as mem::replace
	// input[operator_pos - 1] = Token::Number(value);

	let _ = std::mem::replace(&mut input[operator_pos - 1], Token::Number(value));
	input.drain(operator_pos..=(operator_pos + 1));
}



fn substitute_expression_and_remove_paren(input: &mut Vec<Token>, lparen_pos: usize, rparen_pos: usize, value: f32) {
	let _ = std::mem::replace(&mut input[lparen_pos], Token::Number(value));
	input.drain((lparen_pos + 1)..=rparen_pos);
}



//         vec: [1 + 2 + 3 + 4]
// slice contents:   |   |
fn evaluate_expression(expression_slice: &[Token]) -> f32 {
	let operand_one = unwrap_number_token(&expression_slice[0]);
	let operand_two = unwrap_number_token(&expression_slice[2]);

	// operator
	return match &expression_slice[1] {
		Token::Add => operand_one + operand_two,
		Token::Subtract => operand_one - operand_two,
		Token::Multiply => operand_one * operand_two,
		Token::Divide => operand_one / operand_two,

		_ => panic!("Invalid operation supplied to evaluate_expression!")
	};
}



fn find_token(input: &[Token], token: Token) -> usize {
	return input.iter()
		.position(|element| *element == token)
		.unwrap_or(usize::MAX);
}



fn unwrap_number_token(token: &Token) -> f32 {
	return match token {
		Token::Number(value) => *value,
		_ => panic!("unwrap_number_token called with non-number variant!")
	};
}



#[cfg(test)]
pub fn evaluate_string(expression: &String) -> f32 {
	let mut tokens = super::tokenizer::tokenize(expression);

	return evaluate_tokens(&mut tokens);
}



#[cfg(test)]
#[test]
fn test_evaluator() {
	// basic checks
	assert_eq!(evaluate_string(&String::from("7*3-(1-3)")), 23.0);
	assert_eq!(evaluate_string(&String::from("4/0+1*2")), f32::INFINITY);

	// pemdas
	assert_eq!(evaluate_string(&String::from("4+3*2")), 10.0);
	assert_eq!(evaluate_string(&String::from("3-2-6*6/3")), -11.0);

	// the parentheses bug i never caught
	assert_eq!(evaluate_string(&String::from("(2+2)+3")), 7.0);

	// divide by inf = 0
	assert_eq!(evaluate_string(&String::from("2/(6/0)+7+9/3")), 10.0);
}
