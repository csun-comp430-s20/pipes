mod token;

pub mod tokenizer {
    pub use crate::token::Token;

    pub fn tokenize(input: &str) -> Vec<Token> {
        vec![]
    }

    // helpers
    fn tokenize_int(word: &str) -> Option<Token> {
        None
    }


    fn tokenize_bool(word: &str) -> Option<Token> {
        match word {
            "true" => Some(Token::Bool(true)),
            "false" => Some(Token::Bool(false)),
            _ => None
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
                start_byte= i;
                break;
            }
            else{
                return None;
            }
        }
        for (i, &item) in fullname[start_byte+1..].as_bytes().iter().enumerate(){
            let charat = char::from(item);
             if(charat == '\"'){
                  println!("{}", i+start_byte+1);
                  end_byte = i+start_byte+2;
                  break;
            }
            else{
                return None;
            }
        }
        return Token::Str(&word[start_byte, end_byte]);

    }


    fn tokenize_var_or_keyword(word: &str) -> Option<Token> {
        None
    }


    fn tokenize_syntax(word: &str) -> Option<Token>{
        match word{
            "{" => Some(Token::LeftCurly),
            "}" => Some(Token::RightCurly),
            "(" => Some(Token::LeftParen),
            ")" => Some(Token::RightParen),
            "[" => Some(Token::LeftBrace),
            "]" => Some(Token::RightBrace),
            "." => Some(Token::Dot),
            "," => Some(Token::Comma),
            ";" => Some(Token::Semicolon),
            ":" => Some(Token::Colon),
            "," => Some(Token::Comma),
            "+" => Some(Token::Plus),
            "-" => Some(Token::Minus),
            "/" => Some(Token::Divide),
            "*" => Some(Token::Multiply),
            "%" => Some(Token::Modulo),
            "&&" => Some(Token::And),
            "||" => Some(Token::Or),
            "!" => Some(Token::Not),
            ">" => Some(Token::GreaterThan),
            "<" => Some(Token::LessThan),
            ">=" => Some(Token::GreaterEqual),
            "<=" => Some(Token::LessEqual),
            "==" => Some(Token::Equal),
            "\"!=\"" => Some(Token::Or),
             _ => None
        }
    }

    // takes some iterator and modifies it
    fn skip_whitespace(iter: &mut str) {}
}

#[cfg(test)]
pub mod tests {
    use self::tokenizer;

    #[test]
    fn dummy_test() {
        assert_eq!(true, true)
    }

}
