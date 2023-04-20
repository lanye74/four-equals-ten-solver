use crate::evaluator::tokenizer;



pub fn evaluate(input: String) -> f32 {
	// ownership can be passed here. no need to ref
	let tokens = tokenizer::tokenize(input);

	dbg!(tokens);

	return 3f32;
}
