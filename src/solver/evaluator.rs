use super::tokenizer::{self, Token};



pub fn evaluate(expression: String) -> f32 {
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
			let operation_value = evaluate_expression(&tokens[operator_pos], &tokens[operator_pos - 1], &tokens[operator_pos + 1]);

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

		let operation_value = evaluate_expression(&tokens[operator_pos], &tokens[operator_pos - 1], &tokens[operator_pos + 1]);

		substitute_expression(&mut tokens, operator_pos, operation_value);

		num_expressions -= 1;
	}


	return unwrap_token(&tokens[0]);
}



fn find_next_operator_pos(input: &Vec<Token>, lower_bound: Option<usize>, upper_bound: Option<usize>) -> usize {
	let lower_bound = lower_bound.unwrap_or(0);
	let upper_bound = upper_bound.unwrap_or(input.len() - 1);

	let multiply_pos = find_token_in_range(input, Token::Multiply, lower_bound, upper_bound);
	let divide_pos = find_token_in_range(input, Token::Divide, lower_bound, upper_bound);


	// find which comes first
	let mut operator_pos = std::cmp::min(multiply_pos, divide_pos);

	// if neither is present, search for addition/subtraction
	if operator_pos == usize::MAX {
		let add_pos = find_token_in_range(input, Token::Add, lower_bound, upper_bound);
		let subtract_pos = find_token_in_range(input, Token::Subtract, lower_bound, upper_bound);

		operator_pos = std::cmp::min(add_pos, subtract_pos);
	}

	return operator_pos;
}



fn substitute_expression(input: &mut Vec<Token>, operator_position: usize, value: f32) {
	let mut input_new: Vec<Token> = vec![];
	let input_len = input.len();

	// e.g. 1*(2/3)+4
	// operator_position = 5
	// before_expression = 1*(
	// after_expression = )+4
	let before_expression = &input[0..(operator_position - 1)]; // &input[0..=(operator_position - 2)];
	let after_expression = &input[(operator_position + 2)..input_len];

	input_new.extend_from_slice(before_expression);
	input_new.push(Token::Number(value));
	input_new.extend_from_slice(after_expression);

	*input = input_new;
}



fn remove_parentheses(input: &mut Vec<Token>) {
	let mut input_new: Vec<Token> = vec![];
	let input_len = input.len();

	let lparen_pos = find_token(input, Token::LParen);
	let rparen_pos = find_token(input, Token::RParen);


	let before_paren;

	if lparen_pos != 0 {
		before_paren =  &input[0..=(lparen_pos - 1)];
	} else {
		// throws a hissy fit if paren is at 0
		before_paren = &input[0..0];
	}

	let paren_contents = &input[(lparen_pos + 1)..=(rparen_pos - 1)];
	let after_paren = &input[(rparen_pos + 1)..input_len];


	input_new.extend_from_slice(before_paren);
	input_new.extend_from_slice(paren_contents);
	input_new.extend_from_slice(after_paren);

	*input = input_new;
}



fn evaluate_expression(operation: &Token, operand_one: &Token, operand_two: &Token) -> f32 {
	let operand_one = unwrap_token(operand_one);
	let operand_two = unwrap_token(operand_two);

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



fn find_token(vec: &Vec<Token>, token: Token) -> usize {
	for index in 0..(vec.len()) {
		let element = &vec[index];

		if *element == token {
			return index;
		}
	}

	return usize::MAX;
}



fn find_token_in_range(vec: &Vec<Token>, token: Token, lower_bound: usize, upper_bound: usize) -> usize {
	for index in lower_bound..=upper_bound {
		let element = &vec[index];

		if *element == token {
			return index;
		}
	}

	return usize::MAX;
}



fn unwrap_token(number: &Token) -> f32 {
	return match number {
		Token::Number(value) => *value,
		_ => panic!("unwrap_token called with non-number!")
	};
}



#[cfg(test)]
#[test]
fn test_evaluator() {
	// basic checks
	assert_eq!(evaluate(String::from("7*3-(1-3)")), 23.0);
	assert_eq!(evaluate(String::from("4/0+1*2")), f32::INFINITY);

	// pemdas
	assert_eq!(evaluate(String::from("4+3*2")), 10.0);
	assert_eq!(evaluate(String::from("3-2-6*6/3")), -11.0);

	// the parentheses bug i never caught
	assert_eq!(evaluate(String::from("(2+2)+3")), 7.0)
}
