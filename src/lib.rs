mod token;

pub mod tokenizer {
    pub use crate::token::Token;

    pub fn tokenize(input: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut input = input;

        while input != "" {
            input = skip_whitespace(input);
            let cursor = char::from(input.as_bytes()[0]);

            if cursor == '\"' {
                let (candidate, remainder) = tokenize_str(input);

                if let Some(token) = candidate {
                    tokens.push(token);
                    input = remainder;
                }
            } else if cursor.is_alphanumeric() {
                let (candidate, remainder) = split_first_word(input);
                    if let Some(token) = tokenize_int(candidate) {
                        tokens.push(token);
                        input = remainder;
                    } else { 
                        tokens.push(tokenize_word(candidate));
                        input = remainder;
                    }
            } else {
                if let Some(token) = tokenize_symbol_pair(&input[..1]) {
                    tokens.push(token);
                    input = &input[2..];
                } else if let Some(token) = tokenize_symbol(&input[..0]) {
                    tokens.push(token);
                    input = &input[1..];
                } else {
                    panic!("Failed to parse: {}", &input[..20]);
                }
            }
        }

        tokens
    }

    // helpers
    fn tokenize_int(word: &str) -> Option<Token> {
        match word.parse::<i32>() {
            Ok(ok) => Some(Token::Int(ok)),
            Err(_) => None,
        }
    }

    // takes the full input string
    // returns a string token (or none) and the remainder
    fn tokenize_str(word: &str) -> (Option<Token>, &str) {
        //let user_string = String::from("\"");
        let bytes = word.as_bytes();
        let mut start_byte= 0;
        let mut end_byte = 0;

        (Some(Token::Str((&word[start_byte..end_byte]).to_string())), &word)
    }

    fn tokenize_word(word: &str) -> Token {
        match word {
            "if" => Token::If,
            "elif" => Token::Elif,
            "else" => Token::Else,

            "for" => Token::For,
            "in" => Token::In,
            "while" => Token::While,

            "return" => Token::Return,
            "let" => Token::Let,

            "struct" => Token::Struct,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),

            _ => Token::Var(String::from(word))
        }
    }

    fn tokenize_symbol(sym: &str) -> Option<Token> {
        match sym {
            "=" => Some(Token::Assign),
            "{" => Some(Token::LeftCurly),
            "[" => Some(Token::LeftBrace),
            "(" => Some(Token::LeftParen),
            "}" => Some(Token::RightCurly),
            "]" => Some(Token::RightBrace),
            ")" => Some(Token::RightParen),

            "." => Some(Token::Dot),
            "," => Some(Token::Comma),
            ":" => Some(Token::Colon),
            ";" => Some(Token::Semicolon),

            "-" => Some(Token::Minus),
            "+" => Some(Token::Plus),
            "/" => Some(Token::Divide),
            "*" => Some(Token::Multiply),
            "%" => Some(Token::Modulo),
            "!" => Some(Token::Not),

            ">" => Some(Token::GreaterThan),
            "<" => Some(Token::LessThan),

            _ => None
        }
    }

    fn tokenize_symbol_pair(pair: &str) -> Option<Token> {
        match pair {
            "->" => Some(Token::Output),
            "&&" => Some(Token::And),
            "||" => Some(Token::Or),

            ">=" => Some(Token::GreaterEqual),
            "<=" => Some(Token::LessEqual),
            "==" => Some(Token::Equal),
            "!=" => Some(Token::NotEqual),

            _ => None
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
    pub fn split_first_word(s: &str) -> (&str, &str) {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            let c = char::from(item);

            if !c.is_alphanumeric() && c != '_' {
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
            split_first_word("Hello World"),
            ("Hello", " World"));
    }

    #[test]
    fn get_no_word_1() {
        assert_eq!(
            split_first_word(""),
            ("", ""));
    }

    #[test]
    fn get_no_word_2() {
        assert_eq!(
            split_first_word(" "),
            ("", " "));
    }

    #[test]
    fn get_no_word_3() {
        let s = " cant touch this";
        for _ in 0..100 {
            assert_eq!(split_first_word(s), ("", s));
        }
    }
}
