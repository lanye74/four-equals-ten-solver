use super::tokenizer::{self, Token};



pub fn evaluate(expression: &String) -> f32 {
	let mut tokens: Vec<Token> = tokenizer::tokenize(expression);

	// this function assumes there is only one set of parentheses, and that the input is valid
	// i might write an input validator later. but for now

	// search for parentheses
	let lparen_pos = find_token(&tokens, Token::LParen);

	// if there is a set of parentheses
	if lparen_pos != usize::MAX {
		let mut rparen_pos = find_token(&tokens, Token::RParen);

		// calculate number of expressions to evaluate inside the parentheses
		// position of tokens in parentheses = (lparen + 1, rparen - 1)
		// divide by 2 because yes idk how to articulate it it works
		let mut num_expressions = (rparen_pos - lparen_pos - 2) / 2;

		// loop over every expression
		while num_expressions > 0 {
			// find where the next operator is (mult/div - add/sub)
			let operator_pos = find_next_operator_pos(&tokens, Some(lparen_pos), Some(rparen_pos));

			// compute the expression
			let operation_value = evaluate_expression(&tokens[(operator_pos - 1)..=(operator_pos + 1)]);

			// replace [..., operand_one, operation, operand_two, ...] with [..., result, ...]
			substitute_expression(&mut tokens, operator_pos, operation_value);

			// rparen has moved because of substitution. update it
			rparen_pos = find_token(&tokens, Token::RParen);

			num_expressions -= 1;
		}

		// remove unneeded parentheses
		remove_parentheses(&mut tokens);
	}


	let input_len = tokens.len();

	let mut num_expressions = (input_len - 1) / 2;

	while num_expressions > 0 {
		let operator_pos = find_next_operator_pos(&tokens, None, None);

		let operation_value = evaluate_expression(&tokens[(operator_pos - 1)..=(operator_pos + 1)]);

		substitute_expression(&mut tokens, operator_pos, operation_value);

		num_expressions -= 1;
	}


	return unwrap_token(&tokens[0]);
}



fn find_next_operator_pos(input: &Vec<Token>, lower_bound: Option<usize>, upper_bound: Option<usize>) -> usize {
	let lower_bound = lower_bound.unwrap_or(0);
	let upper_bound = upper_bound.unwrap_or(input.len() - 1);

	let mut multiply_pos = usize::MAX;
	let mut divide_pos = usize::MAX;
	let mut add_pos = usize::MAX;
	let mut subtract_pos = usize::MAX;

	for (index, token) in input[lower_bound..=upper_bound].iter().enumerate() {
		match token {
			Token::Multiply => {
				multiply_pos = index + lower_bound;
				break; // break if found immediately, since mult/div are found first
			}
			Token::Divide => {
				divide_pos = index + lower_bound;
				break;
			}
			Token::Add => {
				if add_pos == usize::MAX { // track only the first
					add_pos = index + lower_bound;
					// don't break since there may be multiplication/division to look for still
				}
			}
			Token::Subtract => {
				if subtract_pos == usize::MAX {
					subtract_pos = index + lower_bound;
				}
			},
			_ => {}
		};
	}


	let mut operator_pos = std::cmp::min(multiply_pos, divide_pos);

	if operator_pos == usize::MAX {
		operator_pos = std::cmp::min(add_pos, subtract_pos);
	}

	return operator_pos;
}



fn substitute_expression(input: &mut Vec<Token>, operator_position: usize, value: f32) {
	// for some reason this is infinitely faster than splice. god knows why
	input.drain((operator_position - 1)..=(operator_position + 1));
	input.insert(operator_position - 1, Token::Number(value));
}



fn remove_parentheses(input: &mut Vec<Token>) {
	// ...this was also unreasonably complicated. oopsie
	input.remove(find_token(input, Token::LParen));
	input.remove(find_token(input, Token::RParen));
}



//          vec: [1 + 2 + 3 + 4]
// slice contents:	 |   |
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



fn find_token(input: &Vec<Token>, token: Token) -> usize {
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
