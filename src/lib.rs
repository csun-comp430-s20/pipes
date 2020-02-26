use crate::Tokens;

pub mod tokenizer
{
	pub fn tokenize(&str) -> Vec<Token> { }

	// helpers
	fn tokenize_int(&str) -> Option<Token.int_tk> { }
	fn tokenize_bool(&str) -> Option<Token.bool_tk> { }
	fn tokenize_str(&str) -> Option<Token.str_tk> { }
	fn tokenize_var_or_keyword(&str) -> Option<Token> { }
	
	// takes some iterator and modifies it
	// fn skip_whitespace(&mut iterator) { }
}

#[cfg(tests)]
pub mod tests
{
	#[test]
	fn dummy_test() {
		assert_eq!(true, true)
	}
}
