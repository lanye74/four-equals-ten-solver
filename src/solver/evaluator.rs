use super::tokenizer::{self, Token};



pub fn evaluate(expression: &String) -> f32 {
	let mut tokens: Vec<Token> = tokenizer::tokenize(expression);

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
			// find where the next operator is (mult/div - add/sub)
			// TODO: possibly find all operator positions in the string, and use an iter to find next
			let operator_pos = if num_expressions == 1 {
				// this is the last operation so its position is already guaranteed
				lparen_pos + 2
			} else {
				// lparen_pos is added to the index to adjust for taking a slice
				find_next_operator_pos(&tokens[lparen_pos..=rparen_pos]) + lparen_pos
			};


			// compute the expression
			let operation_value = evaluate_expression(&tokens[(operator_pos - 1)..=(operator_pos + 1)]);

			// why does this reduce the amount of solutions found for 602793? TODO: compute twice, once with this check, once without; sort, dedup, examine the remaining
			// if operation_value == f32::INFINITY {
			// 	return f32::INFINITY;
			// }

			// replace [..., operand_one, operation, operand_two, ...] with [..., result, ...]
			substitute_expression(&mut tokens, operator_pos, operation_value);

			// rparen has moved because of substitution (this will always be by 2, since substitute_expression replaces one element and removes two). update it
			rparen_pos -= 2;

			num_expressions -= 1;
		}

		// remove unneeded parentheses
		remove_parentheses(&mut tokens, lparen_pos, rparen_pos);
	}


	let input_len = tokens.len();

	let mut num_expressions = (input_len - 1) / 2;

	while num_expressions > 0 {
		let operator_pos = if num_expressions == 1 {
			// this is the last operation so its position is already guaranteed
			1
		} else {
			// lparen_pos is added to the index to adjust for taking a slice
			find_next_operator_pos(&tokens)
		};

		let operation_value = evaluate_expression(&tokens[(operator_pos - 1)..=(operator_pos + 1)]);

		// if operation_value == f32::INFINITY {
		// 	return f32::INFINITY;
		// }

		substitute_expression(&mut tokens, operator_pos, operation_value);

		num_expressions -= 1;
	}


	return unwrap_token(&tokens[0]);
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



fn substitute_expression(input: &mut Vec<Token>, operator_position: usize, value: f32) {
	// this works too, but is cringe and doesn't look nearly as cool as mem::replace
	// input[operator_position - 1] = Token::Number(value);

	let _ = std::mem::replace(&mut input[operator_position - 1], Token::Number(value));
	input.drain(operator_position..=(operator_position + 1));
}



fn remove_parentheses(input: &mut Vec<Token>, lparen_pos: usize, rparen_pos: usize) {
	input.remove(rparen_pos); // remove first as doing lparen first would ajust rparen pos in the vec (also one less element is moved, which is rparen itself)
	input.remove(lparen_pos);
}



//          vec: [1 + 2 + 3 + 4]
// slice contents:   |   |
fn evaluate_expression(expression_slice: &[Token]) -> f32 {
	let operand_one = unwrap_token(&expression_slice[0]);
	let operand_two = unwrap_token(&expression_slice[2]);

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



fn unwrap_token(token: &Token) -> f32 {
	return match token {
		Token::Number(value) => *value,
		_ => panic!("unwrap_token called with non-number!")
	};
}



#[cfg(test)]
#[test]
fn test_evaluator() {
	// basic checks
	assert_eq!(evaluate(&String::from("7*3-(1-3)")), 23.0);
	assert_eq!(evaluate(&String::from("4/0+1*2")), f32::INFINITY);

	// pemdas
	assert_eq!(evaluate(&String::from("4+3*2")), 10.0);
	assert_eq!(evaluate(&String::from("3-2-6*6/3")), -11.0);

	// the parentheses bug i never caught
	assert_eq!(evaluate(&String::from("(2+2)+3")), 7.0)
}
