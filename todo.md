### todo
- ~~generalize tokenizer~~
- ~~eval should handle malformed input~~
- ~~eval should support layered parentheses & multiple pairs~~
	- no need for these. that is beyond the scope of this project and will only slow things down
	- if i really wanted to implement these i should just make a seperate math evaluator project
- option to deduplicate pemdas-equivalent parentheses ("simplify solutions"?)
	- e.g. (1+2)+3+4 = 1+2+3+4. remove parentheses solution
- support command-line args
