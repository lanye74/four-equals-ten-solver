use crate::parser;


pub fn brute_force(input: &Vec<u8>) -> String {
	// dbg!(input);

	// parser::parse(String::new());


	parser::parse("(3*5)/7+0-1*2*9/(8+4)*6".to_string());

	return String::from("");
}
