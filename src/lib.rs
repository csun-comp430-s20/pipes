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

    //Takes a string slice and returns a slice without leading whitespace
    fn skip_whitespace(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            let c = char::from(item);
            if !c.is_whitespace() {
                return &s[i..];
            }
        }

        &s[..]
    }

	// Takes a string slice and returns a slice containing a word and the remainder
    pub fn get_word(s: &str) -> (&str, &str) {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            let c = char::from(item);
            if c.is_whitespace() {
                return (&s[..i], &s[i..]);
            }
        }

        ("", &s[..])
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tokenizer::*;

    #[test]
	fn get_one_word() {
		assert_eq!(
			get_word("Hello World"),
			("Hello", " World"));
	}

	#[test]
	fn get_no_word_1() {
		assert_eq!(
			get_word(""),
			("", ""));
	}

	#[test]
	fn get_no_word_2() {
		assert_eq!(
			get_word(" "),
			("", " "));
	}

	#[test]
	fn get_no_word_3() {
		let s = " cant touch this";
		for _ in 0..100 {
			assert_eq!(get_word(s), ("", s));
		}
	}
}
