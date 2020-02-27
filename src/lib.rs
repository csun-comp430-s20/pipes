mod token;

pub mod tokenizer {
    pub use crate::token::Token;

    pub fn tokenize(input: &str) -> Vec<Token> {
        vec![]
    }

    // helpers
    fn tokenize_int(word: &str) -> Option<Token> {
        let test = word.parse::<i32>();

	    match test {
	        Ok(ok) => Some(Token::Int(ok)),
	        Err(e) => None,
        }
    }
    fn tokenize_bool(word: &str) -> Option<Token> {
        None
    }
    fn tokenize_str(word: &str) -> Option<Token> {
        None
    }
    fn tokenize_var_or_keyword(word: &str) -> Option<Token> {
        match word {
	        "if" => Some(Token::If),
	        "elif" => Some(Token::Elif),
	        "else" => Some(Token::Else),
	        "for" => Some(Token::For),
	        "in" => Some(Token::In),
	        "while" => Some(Token::While),
	        "return" => Some(Token::Return),
	        "let" => Some(Token::Let),
	        "struct" => Some(Token::Struct),
	        _ => Some(Token::Var(word.to_string())),
	    }
    }

    // takes some iterator and modifies it
    fn skip_whitespace(iter: &mut str) {}
}

#[cfg(tests)]
pub mod tests {
    use self::tokenizer;

    #[test]
    fn dummy_test() {
        assert_eq!(true, true)
    }
}
