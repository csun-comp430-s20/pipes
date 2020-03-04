mod token;
use crate::token::Token;

pub mod tokenizer {
    use super::*;
    pub use crate::token::Token;

    pub fn tokenize(input: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut input = input;

        while input != "" {
            input = skip_whitespace(input);
            let cursor = char::from(input.as_bytes()[0]);

            if cursor == '\"' {
                if let Some((token, remainder)) = tokenize_str(&input[1..]) {
                    tokens.push(token);
                    input = remainder;
                } else {
                    panic!("No closing double-quote: {}", &input[..20])
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
                if let Some(token) = tokenize_symbol_pair(&input[..2]) {
                    tokens.push(token);
                    input = &input[2..];
                } else if let Some(token) = tokenize_symbol(&input[..1]) {
                    tokens.push(token);
                    input = &input[1..];
                } else {
                    panic!("Failed to parse: {}", &input[..20]);
                }
            }
        }

        tokens
    }
}

// helpers
fn tokenize_int(word: &str) -> Option<Token> {
    match word.parse::<i32>() {
        Ok(ok) => Some(Token::Int(ok)),
        Err(_) => None,
    }
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

        _ => Token::Var(String::from(word)),
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

        _ => None,
    }
}

// takes an input string and returns a string token (or none) and the remainder
fn tokenize_str(s: &str) -> Option<(Token, &str)> {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        let c = char::from(item);

        if c == '\"' {
            return Some((Token::Str(String::from(&s[..i])), &s[i + 1..]));
        }
    }

    None
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

fn tokenize_symbol_pair(pair: &str) -> Option<Token> {
    match pair {
        "->" => Some(Token::Output),
        "&&" => Some(Token::And),
        "||" => Some(Token::Or),

        ">=" => Some(Token::GreaterEqual),
        "<=" => Some(Token::LessEqual),
        "==" => Some(Token::Equal),
        "!=" => Some(Token::NotEqual),

        _ => None,
    }
}

//takes a string slice and returns a slice without leading whitespace
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

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tokenizer::*;

    #[test]
    fn tokenize_nothing() {
        assert_eq!(tokenize(""), vec!())
    }

    #[test]
    fn tokenize_int_assignment() {
        assert_eq!(
            tokenize("let x: int = 32;"),
            vec![
                Token::Let,
                Token::Var(String::from("x")),
                Token::Colon,
                Token::Assign,
                Token::Int(32),
                Token::Semicolon,
            ]
        )
    }

    #[test]
    fn tokenize_str_one_word() {
        assert_eq!(
            tokenize_str("Hello\""),
            Some((Token::Str(String::from("Hello")), ""))
        )
    }

    #[test]
    fn tokenize_str_two_words() {
        assert_eq!(
            tokenize_str("Hello World\""),
            Some((Token::Str(String::from("Hello World")), ""))
        )
    }

    #[test]
    fn tokenize_str_words_and_remainder() {
        assert_eq!(
            tokenize_str("Hello World\"; let x = 5;"),
            Some((Token::Str(String::from("Hello World")), "; let x = 5;"))
        )
    }

    #[test]
    fn tokenize_str_empty() {
        assert_eq!(tokenize_str(""), None)
    }

    #[test]
    fn tokenize_not_a_string() {
        assert_eq!(tokenize_str("let x;"), None)
    }

    //re-write as test for tokenize()
    //#[test]
    //fn tokenize_str_has_string_later() {
    //assert_eq!(tokenize_str("let x = \"ab\";"), (None, "let x = \"ab\";"))
    //}

    #[test]
    fn tokenize_lots_of_ints() {
        for x in -500..500 {
            let s = x.to_string();
            assert_eq!(tokenize_int(&s), Some(Token::Int(x)))
        }
    }

    #[test]
    fn tokenize_not_an_int() {
        assert_eq!(tokenize_int("let x = 5;"), None)
    }

    #[test]
    fn skip_whitespace_empty() {
        assert_eq!(skip_whitespace(""), "")
    }

    #[test]
    fn skip_whitespace_no_whitespace() {
        assert_eq!(skip_whitespace("HelloWorld"), "HelloWorld")
    }

    #[test]
    fn skip_whitespace_proper_sentence() {
        assert_eq!(
            skip_whitespace("The mitochondria is the powerhouse of the cell!"),
            "The mitochondria is the powerhouse of the cell!"
        )
    }

    #[test]
    fn skip_whitespace_single_space() {
        assert_eq!(skip_whitespace(" word"), "word")
    }

    #[test]
    fn skip_whitespace_many_spaces() {
        assert_eq!(skip_whitespace("       word"), "word")
    }

    #[test]
    fn skip_whitespace_tabs() {
        assert_eq!(skip_whitespace("		word"), "word")
    }

    #[test]
    fn skip_whitespace_newlines() {
        assert_eq!(
            skip_whitespace(
                "



                            word"
            ),
            "word"
        )
    }

    #[test]
    fn get_one_word() {
        assert_eq!(split_first_word("Hello World"), ("Hello", " World"));
    }

    #[test]
    fn get_no_word_1() {
        assert_eq!(split_first_word(""), ("", ""));
    }

    #[test]
    fn get_no_word_2() {
        assert_eq!(split_first_word(" "), ("", " "));
    }

    #[test]
    fn get_no_word_3() {
        let s = " cant touch this";
        for _ in 0..100 {
            assert_eq!(split_first_word(s), ("", s));
        }
    }

    #[test]
    fn curly_token_test() {
        let received_tokens: Vec<Token> = tokenize("{");
        assert_eq!(received_tokens, vec![Token::LeftCurly]);
    }

    #[test]
    fn if_token_test() {
        let received_tokens: Vec<Token> = tokenize("if");
        assert_eq!(received_tokens, vec!(Token::If));
    }

    #[test]
    fn else_token_test() {
        let received_tokens: Vec<Token> = tokenize("else");
        assert_eq!(received_tokens, vec!(Token::Else));
    }

    #[test]
    fn else_if_token_test() {
        let received_tokens: Vec<Token> = tokenize("else if");
        assert_eq!(received_tokens, vec!(Token::Else, Token::If));
    }

    #[test]
    fn int_token_test() {
        let received_tokens: Vec<Token> = tokenize("12");
        assert_eq!(received_tokens, vec!(Token::Int(12)));
    }

    #[test]
    fn negative_int_token_test() {
        let received_tokens: Vec<Token> = tokenize("-5");
        assert_eq!(received_tokens, vec!(Token::Int(-5)));
    }

    #[test]
    fn false_token_test() {
        let received_tokens: Vec<Token> = tokenize("false");
        assert_eq!(received_tokens, vec!(Token::Bool(false)));
    }
}
