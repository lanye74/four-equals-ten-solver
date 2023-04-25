// use crate::evaluator::evaluator;



pub fn brute_force(input: &Vec<u8>) -> String {
	// evaluator::evaluate("(3*5)/7+0-1*2*9/(8+4)*6".to_string());


	// dbg!(evaluator::evaluate(String::from("6*(1/9*9+2)+3/2")));
	// permutate(input.len(), input);

	// dbg!(input);

	// use heap's algorithm here
	generate_permutations(&vec![1, 2, 3, 4]);


	return String::from("");
}



fn generate_permutations(input: &Vec<u8>) -> Vec<Vec<u8>> {
	let output: Vec<Vec<u8>> = vec![];


	// let indices =
	// for i in 0..(factorial(input.len())) {

	return output;
}



/*
1 2 3 4
1 2 4 3
1 3 2 4
1 3 4 2
1 4 2 3
1 4 3 2

2 1 3 4
2 1 4 3
2 3 1 4
2 3 4 1
2 4 1 3
2 4 3 1

3 1 2 4
3 1 4 2
3 2 1 4
3 2 4 1
3 4 1 2
3 4 2 1

4 1 2 3
4 1 3 2
4 2 1 3
4 2 3 1
4 3 1 2
4 3 2 1
*/



// fn factorial(of: usize) -> usize {
// 	let mut product: usize = 1;

// 	for i in 2..=of {
// 		product *= i;
// 	}

// 	return product;
// }
