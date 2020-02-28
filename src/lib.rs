mod token;

pub mod tokenizer {
    pub use crate::token::Token;

    pub fn tokenize(input: &str) -> Vec<Token> {
		let tokens: Vec<Token> = vec![];
		while input != "" {
			input = skip_whitespace(input);
			let (tk, input) = tokenize_str(input);
			if let Some(tk) = tk {
				tokens.push(tk);
				continue;
			}
			let (word, input) = get_word(input);
			if let Some(tk) = tokenize_int(word) {
				tokens.push(tk);
			}
			else {
				tokens.push(tokenize_syntax(word));
			}
		}
		tokens
    }

    // helpers
    fn tokenize_int(word: &str) -> Option<Token> {
	    match word.parse::<i32>() {
	        Ok(ok) => Some(Token::Int(ok)),
	        Err(e) => None,
        }
    }

	// takes the full input string
	// returns a string token (or none) and the remainder
    fn tokenize_str(input: &str) -> (Option<Token>, &str) {
        let bytes = input.as_bytes();
		if char::from(bytes[0]) != '\"' {
			return (None, input);
		}

        for (i, &item) in bytes.iter().enumerate() {
            let c = char::from(item);
            if c.is_whitespace() {
                return (Some(Token::Str(String::from(&input[..i]))), &input[i..]);
            }
        }
        (None, &input[..])
    }

    fn tokenize_syntax(word: &str) -> Token {
        match word {
			"if" => Token::If,
			"elif" => Token::Elif,
			"else" => Token::Else,

			"for" => Token::For,
			"in" => Token::In,
			"while" => Token::While,

			"return" => Token::Return,
			"->" => Token::Output,
			"let" => Token::Let,
			"=" => Token::Assign,

			"struct" => Token::Struct,
			"true" => Token::Bool(true),
			"false" => Token::Bool(false),

            "{" => Token::LeftCurly,
            "[" => Token::LeftBrace,
            "(" => Token::LeftParen,
            "}" => Token::RightCurly,
            "]" => Token::RightBrace,
            ")" => Token::RightParen,

            "." => Token::Dot,
            "," => Token::Comma,
            ":" => Token::Colon,
            ";" => Token::Semicolon,

            "-" => Token::Minus,
            "+" => Token::Plus,
            "/" => Token::Divide,
            "*" => Token::Multiply,
            "%" => Token::Modulo,

            "&&" => Token::And,
            "||" => Token::Or,
            "!" => Token::Not,

            ">" => Token::GreaterThan,
            "<" => Token::LessThan,
            ">=" => Token::GreaterEqual,
            "<=" => Token::LessEqual,
            "==" => Token::Equal,
            "!=" => Token::NotEqual,

             _ => Token::Var(String::from(word)),
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
