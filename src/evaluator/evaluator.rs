use super::tokenizer;
use super::parser;



pub fn evaluate(input: String) -> f32 {
	// ownership can be passed here. no need to ref
	let tokens = tokenizer::tokenize(input);

	let output = parser::parse(tokens);

	return output;
}
