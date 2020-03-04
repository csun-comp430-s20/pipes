mod token;
use crate::token::Token;

pub mod tokenizer {
    pub use crate::token::Token;
	use super::*;

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
}

fn tokenize_int(word: &str) -> Option<Token> {
	match word.parse::<i32>() {
		Ok(ok) => Some(Token::Int(ok)),
		Err(_) => None,
	}
}

// takes the full input string
// returns a string token (or none) and the remainder
fn tokenize_str(word: &str) -> (Option<Token>, &str) {;
	let bytes = word.as_bytes();
	let mut start_byte= 0;
	let mut end_byte = 0;
    if  word == "" {
        return (None, word);
    }
    else{
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
	if start_byte >= 0{
		for (i, &item) in word[start_byte+1..].as_bytes().iter().enumerate(){
			let charat = char::from(item);
			 if(charat == '\"'){
				  end_byte = i+start_byte+2;
				  break;
			}
		}
	}

    let x = Some(Token::Str((&word[start_byte..end_byte]).to_string()));
    if (word[start_byte..end_byte].to_string() == ""){
        return (Some(Token::Str(word.to_string())), "");
    }
    else{
	return (x, &word[end_byte..]);
    }
}
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
fn get_word(s: &str) -> (&str, &str) {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		let c = char::from(item);
		if c.is_whitespace() {
			return (&s[..i], &s[i..]);
		}
	}
	(&s[..], "")
}

#[cfg(test)]
pub mod tests {
	use crate::tokenizer::*;
    use super::*;

	#[test]
	fn tokenize_nothing() {
		assert_eq!(
			tokenize(""),
			vec!())
	}

	#[test]
	fn tokenize_int_assignment() {
		assert_eq!(
			tokenize("let x: int = 32;"),
			vec![Token::Let,
				 Token::Var(String::from("x")),
				 Token::Colon,
				 Token::Assign,
				 Token::Int(32),
				 Token::Semicolon,
			])
		}

	#[test]
	fn tokenize_str_one_word() {
		let word = "\"Hello\"";
		assert_eq!(
			tokenize_str(word),
			(Some(Token::Str(String::from(word))),
			""))
	}

	#[test]
	fn tokenize_str_two_words() {
		let word = "\"Hello World\"";
		assert_eq!(
			tokenize_str(word),
			(Some(Token::Str(String::from(word))),
			""))
	}

	#[test]
	fn tokenize_str_words_and_remainder() {
		let input = "\"Hello World\"; let x = 5;";
		assert_eq!(
			tokenize_str(input),
			(Some(Token::Str(String::from("\"Hello World\""))),
			"; let x = 5;"))
	}

	#[test]
	fn tokenize_str_empty() {
		assert_eq!(
			tokenize_str(""),
			(None, ""))
	}

	#[test]
	fn tokenize_not_a_string() {
		assert_eq!(
			tokenize_str("let x;"),
			(None, "let x;"))
	}

	#[test]
	fn tokenize_str_has_string_later() {
		assert_eq!(
			tokenize_str("let x = \"ab\";"),
			(None, "let x = \"ab\";"))
	}

	#[test]
	fn tokenize_lots_of_ints() {
		for x in -500..500 {
			let s = x.to_string();
			assert_eq!(
				tokenize_int(&s),
				Some(Token::Int(x)))
		}
	}

	#[test]
	fn tokenize_not_an_int() {
		assert_eq!(
			tokenize_int("let x = 5;"),
			None)
	}

	#[test]
	fn skip_whitespace_empty() {
		assert_eq!(
			skip_whitespace(""),
			"")
	}

	#[test]
	fn skip_whitespace_no_whitespace() {
		assert_eq!(
			skip_whitespace("HelloWorld"),
			"HelloWorld")
	}

	#[test]
	fn skip_whitespace_proper_sentence() {
		assert_eq!(
			skip_whitespace("The mitochondria is the powerhouse of the cell!"),
			"The mitochondria is the powerhouse of the cell!")
	}

	#[test]
	fn skip_whitespace_single_space() {
		assert_eq!(
			skip_whitespace(" word"),
			"word")
	}

	#[test]
	fn skip_whitespace_many_spaces() {
		assert_eq!(
			skip_whitespace("       word"),
			"word")
	}

	#[test]
	fn skip_whitespace_tabs() {
		assert_eq!(
			skip_whitespace("		word"),
			"word")
	}

	#[test]
	fn skip_whitespace_newlines() {
		assert_eq!(
			skip_whitespace("



							word"),
			"word")
	}

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

	#[test]
	fn curly_token_test() {
	    let receivedTokens: Vec<Token> = tokenize("{");
	    assert!(receivedTokens[0] == Token::LeftCurly);
	}

	#[test]
	fn if_token_test() {
	    let receivedTokens: Vec<Token> = tokenize("if");
	    assert!(receivedTokens[0] == Token::If);
	}

	#[test]
	fn else_token_test() {
	    let receivedTokens: Vec<Token> = tokenize("else");
	    assert!(receivedTokens[0] == Token::Else);
	}

	#[test]
	fn else_if_token_test() {
	    let receivedTokens: Vec<Token> = tokenize("else if");
	    assert!(receivedTokens[0] == Token::Else && receivedTokens[1] == Token::If);
	}

	#[test]
	fn int_token_test() {
	    let receivedTokens: Vec<Token> = tokenize("12");
	    assert!(receivedTokens[0] == Token::Int(12));
	}

	#[test]
	fn negative_int_token_test() {
	    let receivedTokens: Vec<Token> = tokenize("-5");
	    assert!(receivedTokens[0] == Token::Int(-5));
	}

	#[test]
	fn false_token_test() {
	    let receivedTokens: Vec<Token> = tokenize("false");
	    assert!(receivedTokens[0] == Token::Bool(false));
	}
}
