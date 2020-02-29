mod token;

pub mod tokenizer {
    pub use crate::token::Token;

    pub fn tokenize(input: &str) -> Vec<Token> {
		let mut tokens: Vec<Token> = vec![];
		let mut input = input;

		while input != "" {
			input = skip_whitespace(input);
			let (tk, remainder) = tokenize_str(input);
			input = remainder;
			if let Some(tk) = tk {
				tokens.push(tk);
				continue;
			}

			let (word, remainder) = get_word(input);
			input = remainder;
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
    fn tokenize_str(word: &str) -> (Option<Token>, &str) {
        let mut user_string = String::from("\"");
        let mut bytes = word.as_bytes();
        let mut start_byte= 0;
        let mut end_byte = 0;
        for (i, &item) in bytes.iter().enumerate(){
            let charat = char::from(item);
            if(charat == '\"'){
                start_byte = i;
                break;
            }
            else{
                return (None, word);
            }
        }
        if start_byte > 0{
            for (i, &item) in word[start_byte+1..].as_bytes().iter().enumerate(){
                let charat = char::from(item);
                 if(charat == '\"'){
                      println!("{}", i+start_byte+1);
                      end_byte = i+start_byte+2;
                      break;
                }
                else{
                    return (None, word);
                }
            }
        }
        &word[end_byte+1..].to_string().to_owned();
        return (Some(Token::Str((&word[start_byte..end_byte]).to_string())),
                &word);
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
