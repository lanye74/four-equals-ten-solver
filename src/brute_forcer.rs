use crate::evaluator::evaluator;



pub fn brute_force(_input: &Vec<u8>) -> String {
	// evaluator::evaluate("(3*5)/7+0-1*2*9/(8+4)*6".to_string());
	dbg!(evaluator::evaluate(String::from("6*(1/9*9+2)+3/2")));

	return String::from("");
}
