mod token;
use crate::token::*;

pub mod tokenizer {
    use super::*;
    pub use crate::token::*;

    pub fn tokenize(input: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut input = input;

		while let Some(chunk) = make_chunk(input) {
			// it'd be easier to tokenize string here
			if let Some(tk) = tokenize_int(chunk.alphanumerics) {
				tokens.push(tk);
			} else if let Some(tk) = tokenize_word(chunk.alphanumerics) {
				tokens.push(tk);
			}
			else if !chunk.alphanumerics.is_empty() {
				panic!("Failed to tokenize '{}' as alphanumeric", chunk.alphanumerics);
			}

			if chunk.symbols.is_empty() {
				// make sure to do nothing on empty
				()
			} else if let Some(tk) = tokenize_symbol(&chunk.symbols) {
				tokens.push(tk);
			} else if let (Some(tk1), Some(tk2)) = (
					tokenize_symbol(&chunk.symbols[..1]),
					tokenize_symbol(&chunk.symbols[1..])) {
				tokens.push(tk1);
				tokens.push(tk2);
			} else {
				panic!("Failed to tokenize '{}' as symbol", chunk.symbols);
			}

			input = if !chunk.remainder.is_empty() && &chunk.remainder[..1] == "\"" {
				let (tk, rem) = tokenize_str(&chunk.remainder[1..]);
				tokens.push(tk);
				rem
			} else {
				chunk.remainder
			}
		}
		tokens
	}
}

#[derive(Debug, Eq, PartialEq)]
struct OrderedChunk<'a> {
	alphanumerics: &'a str,
	symbols: &'a str,
	remainder: &'a str,
}

fn make_chunk(input: &str) -> Option<OrderedChunk> {
	if input == "" { return None; }
	let (mut start, bytes) = (0, input.as_bytes());

	while start < bytes.len()
		&& char::from(bytes[start]).is_whitespace() {
			start += 1;
	}
	if start == bytes.len() { return None; }

	let mut anum = start;
	while anum < bytes.len()
		&& (char::from(bytes[anum]).is_alphanumeric()
		|| char::from(bytes[anum]) == '_') {
			anum += 1;
	}

	let mut sym = anum;
	while sym < bytes.len() && sym < anum + 2
		&& !(char::from(bytes[sym]).is_alphanumeric()
		|| char::from(bytes[sym]) == '_'
		|| char::from(bytes[sym]) == '\"'
		|| char::from(bytes[sym]).is_whitespace()) {
			sym += 1;
	}

	Some(OrderedChunk {
		alphanumerics: &input[start..anum],
		symbols: &input[anum..sym],
		remainder: &input[sym..],
	})
}

fn tokenize_int(word: &str) -> Option<Token> {
    match word.replace("_", "").parse::<i32>() {
        Ok(int) => Some(Token::Int(int)),
        Err(_) => None,
    }
}

// assumes that a starting double quote was already found
fn tokenize_str(input: &str) -> (Token, &str) {
    for (i, &item) in input.as_bytes().iter().enumerate() {
        if char::from(item) == '\"' {
            return (Token::Str(String::from(&input[..i])), &input[i + 1..]);
        }
    }
	panic!("Failed to tokenize {} as a string, no closing double quote", input)
}

fn tokenize_word(word: &str) -> Option<Token> {
    match word {
        "if" => Some(Token::If),
        "elif" => Some(Token::Elif),
        "else" => Some(Token::Else),

        "for" => Some(Token::For),
        "in" => Some(Token::In),
        "while" => Some(Token::While),

		"func" => Some(Token::Function),
        "return" => Some(Token::Return),
        "let" => Some(Token::Let),

        "struct" => Some(Token::Struct),
        "true" => Some(Token::Bool(true)),
        "false" => Some(Token::Bool(false)),

        "int" => Some(Token::TypeName(Type::Int)),
        "str" => Some(Token::TypeName(Type::Str)),
        "bool" => Some(Token::TypeName(Type::Bool)),
        "void" => Some(Token::TypeName(Type::Void)),

		"" => None,
        _ => Some(Token::Var(String::from(word))),
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

        "->" => Some(Token::Output),
        "&&" => Some(Token::And),
        "||" => Some(Token::Or),

        ">=" => Some(Token::GreaterEqual),
        "<=" => Some(Token::LessEqual),
        "==" => Some(Token::Equal),
        "!=" => Some(Token::NotEqual),

		"\"\"" => Some(Token::Str(String::new())),
        _ => None,
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tokenizer::*;

    // ----------- tokenize() tests ---------- \\
    // ----------- simple input tests ---------- \\
    #[test]
    fn tokenize_nothing() {
        assert_eq!(tokenize(""), vec![])
    }

    #[test]
    fn tokenize_keyword_if() {
        assert_eq!(tokenize("if"), vec![Token::If])
    }

    #[test]
    fn tokenize_keyword_elif() {
        assert_eq!(tokenize("elif"), vec![Token::Elif])
    }

    #[test]
    fn tokenize_keyword_else() {
        assert_eq!(tokenize("else"), vec![Token::Else])
    }

    #[test]
    fn tokenize_keyword_for() {
        assert_eq!(tokenize("for"), vec![Token::For])
    }

    #[test]
    fn tokenize_keyword_in() {
        assert_eq!(tokenize("in"), vec![Token::In])
    }

    #[test]
    fn tokenize_keyword_while() {
        assert_eq!(tokenize("while"), vec![Token::While])
    }

    #[test]
    fn tokenize_keyword_function() {
        assert_eq!(tokenize("func"), vec![Token::Function])
    }

    #[test]
    fn tokenize_keyword_output() {
        assert_eq!(tokenize("->"), vec![Token::Output])
    }

    #[test]
    fn tokenize_keyword_return() {
        assert_eq!(tokenize("return"), vec![Token::Return])
    }

    #[test]
    fn tokenize_keyword_let() {
        assert_eq!(tokenize("let"), vec![Token::Let])
    }

    #[test]
    fn tokenize_keyword_assign() {
        assert_eq!(tokenize("="), vec![Token::Assign])
    }

    #[test]
    fn tokenize_keyword_type_void() {
        assert_eq!(tokenize("void"), vec![Token::TypeName(Type::Void)])
    }

    #[test]
    fn tokenize_keyword_type_int() {
        assert_eq!(tokenize("int"), vec![Token::TypeName(Type::Int)])
    }

    #[test]
    fn tokenize_keyword_type_str() {
        assert_eq!(tokenize("str"), vec![Token::TypeName(Type::Str)])
    }

    #[test]
    fn tokenize_keyword_type_bool() {
        assert_eq!(tokenize("bool"), vec![Token::TypeName(Type::Bool)])
    }

    #[test]
    fn tokenize_keyword_type_struct() {
        // not sure about this one
        assert_eq!(
            tokenize("struct foo"),
            vec![Token::Struct, Token::Var(String::from("foo"))]
        )
    }

    #[test]
    fn tokenize_keyword_struct() {
        assert_eq!(tokenize("struct"), vec![Token::Struct])
    }

    #[test]
    fn tokenize_keyword_int() {
        assert_eq!(tokenize("5"), vec![Token::Int(5)])
    }

    #[test]
    fn tokenize_keyword_negative_int() {
        // the parser will firgure out that it's a negative int
        assert_eq!(tokenize("-5"), vec![Token::Minus, Token::Int(5)])
    }

    #[test]
    fn tokenize_keyword_true() {
        assert_eq!(tokenize("true"), vec![Token::Bool(true)])
    }

    #[test]
    fn tokenize_keyword_false() {
        assert_eq!(tokenize("false"), vec![Token::Bool(false)])
    }

    #[test]
    fn tokenize_keyword_left_curly() {
        assert_eq!(tokenize("{"), vec![Token::LeftCurly])
    }

    #[test]
    fn tokenize_keyword_right_curly() {
        assert_eq!(tokenize("}"), vec![Token::RightCurly])
    }

    #[test]
    fn tokenize_keyword_left_brace() {
        assert_eq!(tokenize("["), vec![Token::LeftBrace])
    }

    #[test]
    fn tokenize_keyword_right_brace() {
        assert_eq!(tokenize("]"), vec![Token::RightBrace])
    }

    #[test]
    fn tokenize_keyword_left_paren() {
        assert_eq!(tokenize("("), vec![Token::LeftParen])
    }

    #[test]
    fn tokenize_keyword_right_paren() {
        assert_eq!(tokenize(")"), vec![Token::RightParen])
    }

    #[test]
    fn tokenize_keyword_dot() {
        assert_eq!(tokenize("."), vec![Token::Dot])
    }

    #[test]
    fn tokenize_keyword_comma() {
        assert_eq!(tokenize(","), vec![Token::Comma])
    }

    #[test]
    fn tokenize_keyword_colon() {
        assert_eq!(tokenize(":"), vec![Token::Colon])
    }

    #[test]
    fn tokenize_keyword_semicolon() {
        assert_eq!(tokenize(";"), vec![Token::Semicolon])
    }

    #[test]
    fn tokenize_keyword_minus() {
        assert_eq!(tokenize("-"), vec![Token::Minus])
    }

    #[test]
    fn tokenize_keyword_plus() {
        assert_eq!(tokenize("+"), vec![Token::Plus])
    }

    #[test]
    fn tokenize_keyword_divide() {
        assert_eq!(tokenize("/"), vec![Token::Divide])
    }

    #[test]
    fn tokenize_keyword_multiply() {
        assert_eq!(tokenize("*"), vec![Token::Multiply])
    }

    #[test]
    fn tokenize_keyword_modulo() {
        assert_eq!(tokenize("%"), vec![Token::Modulo])
    }

    #[test]
    fn tokenize_keyword_and() {
        assert_eq!(tokenize("&&"), vec![Token::And])
    }

    #[test]
    fn tokenize_keyword_or() {
        assert_eq!(tokenize("||"), vec![Token::Or])
    }

    #[test]
    fn tokenize_keyword_not() {
        assert_eq!(tokenize("!"), vec![Token::Not])
    }

    #[test]
    fn tokenize_keyword_greater_than() {
        assert_eq!(tokenize(">"), vec![Token::GreaterThan])
    }

    #[test]
    fn tokenize_keyword_less_than() {
        assert_eq!(tokenize("<"), vec![Token::LessThan])
    }

    #[test]
    fn tokenize_keyword_greater_equal() {
        assert_eq!(tokenize(">="), vec![Token::GreaterEqual])
    }

    #[test]
    fn tokenize_keyword_less_equal() {
        assert_eq!(tokenize("<="), vec![Token::LessEqual])
    }

    #[test]
    fn tokenize_keyword_equal() {
        assert_eq!(tokenize("=="), vec![Token::Equal])
    }

    #[test]
    fn tokenize_keyword_not_equal() {
        assert_eq!(tokenize("!="), vec![Token::NotEqual])
    }

    #[test]
    fn tokenize_weird_int() {
        assert_eq!(tokenize("1_000_000"), vec![Token::Int(1000000)])
    }

    #[test]
    fn tokenize_weird_var_name_1() {
        assert_eq!(tokenize("ifelse"), vec![Token::Var(String::from("ifelse"))])
    }

    #[test]
    fn tokenize_weird_var_name_2() {
        assert_eq!(tokenize("If"), vec![Token::Var(String::from("If"))])
    }

    #[test]
    fn tokenize_weird_var_name_3() {
        assert_eq!(tokenize("_"), vec![Token::Var(String::from("_"))])
    }

    #[test]
    fn tokenize_weird_var_name_4() {
        assert_eq!(
            tokenize("under_score"),
            vec![Token::Var(String::from("under_score"))]
        )
    }

    #[test]
    fn tokenize_weird_var_name_5() {
        assert_eq!(
            tokenize("_underscore"),
            vec![Token::Var(String::from("_underscore"))]
        )
    }

    #[test]
    fn tokenize_weird_var_name_6() {
        assert_eq!(
            tokenize("underscore_"),
            vec![Token::Var(String::from("underscore_"))]
        )
    }

    #[test]
    fn tokenize_weird_var_name_7() {
        assert_eq!(tokenize("num1"), vec![Token::Var(String::from("num1"))])
    }

    #[test]
    fn tokenize_weird_var_name_8() {
        assert_eq!(tokenize("1num"), vec![Token::Var(String::from("1num"))])
    }

    #[test]
    fn tokenize_weird_var_name_9() {
        assert_eq!(tokenize("123_this_is_a_var"), vec![Token::Var(String::from("123_this_is_a_var"))])
    }

    #[test]
	#[should_panic]
    fn tokenize_illegal_var_name_1() {
        assert_eq!(tokenize("&"), vec![])
    }

    #[test]
	#[should_panic]
    fn tokenize_illegal_var_name_2() {
        assert_eq!(tokenize("|||"), vec![])
    }

    #[test]
	#[should_panic]
    fn tokenize_illegal_var_name_3() {
        assert_eq!(tokenize("?"), vec![])
    }

    // ----------- basic input tests ---------- \\
    #[test]
    fn tokenize_1_plus_2_no_whitespace() {
        assert_eq!(
            tokenize("1+2"),
            vec![Token::Int(1), Token::Plus, Token::Int(2),]
        )
    }

    #[test]
    fn tokenize_1_plus_2_single_space() {
        assert_eq!(
            tokenize("1 + 2"),
            vec![Token::Int(1), Token::Plus, Token::Int(2),]
        )
    }

    #[test]
    fn tokenize_1_plus_2_single_space_front() {
        assert_eq!(
            tokenize(" 1+2"),
            vec![Token::Int(1), Token::Plus, Token::Int(2),]
        )
    }

    #[test]
    fn tokenize_1_plus_2_single_tab_front() {
        assert_eq!(
            tokenize("	1+2"),
            vec![Token::Int(1), Token::Plus, Token::Int(2),]
        )
    }

    #[test]
    fn tokenize_1_plus_2_single_newline_front() {
        assert_eq!(
            tokenize(
                "
					 1+2"
            ),
            vec![Token::Int(1), Token::Plus, Token::Int(2),]
        )
    }

    #[test]
    fn tokenize_1_plus_2_single_space_back() {
        assert_eq!(
            tokenize("1+2 "),
            vec![Token::Int(1), Token::Plus, Token::Int(2),]
        )
    }

    #[test]
    fn tokenize_1_plus_2_single_tab_back() {
        assert_eq!(
            tokenize("1+2	"),
            vec![Token::Int(1), Token::Plus, Token::Int(2),]
        )
    }

    #[test]
    fn tokenize_1_plus_2_single_newline_back() {
        assert_eq!(
            tokenize(
                "1+2
					 "
            ),
            vec![Token::Int(1), Token::Plus, Token::Int(2),]
        )
    }

    #[test]
    fn tokenize_1_plus_2_single_space_wrapped() {
        assert_eq!(
            tokenize(" 1 + 2 "),
            vec![Token::Int(1), Token::Plus, Token::Int(2),]
        )
    }

    #[test]
    fn tokenize_1_plus_2_too_much_whitespace() {
        assert_eq!(
            tokenize(
                "	1
					 +
					 2						 "
            ),
            vec![Token::Int(1), Token::Plus, Token::Int(2),]
        )
    }

	#[test]
	fn tokenize_symbol_as_string_1() {
		assert_eq!(tokenize("\"(\""), vec![Token::Str(String::from("("))])
	}

	#[test]
	fn tokenize_symbol_as_string_2() {
		assert_eq!(tokenize("\"()\""), vec![Token::Str(String::from("()"))])
	}

	#[test]
	fn tokenize_symbol_as_string_3() {
		assert_eq!(tokenize("(\"\""), vec![Token::LeftParen, Token::Str(String::from(""))])
	}

	#[test]
	fn tokenize_symbol_as_string_4() {
		assert_eq!(
			tokenize("(\"\")"),
			vec![
				Token::LeftParen,
				Token::Str(String::from("")),
				Token::RightParen,
			])
	}

    // ----------- typical input tests ---------- \\
    #[test]
    fn tokenize_int_assignment() {
        assert_eq!(
            tokenize("let x: int = 32;"),
            vec![
                Token::Let,
                Token::Var(String::from("x")),
                Token::Colon,
                Token::TypeName(Type::Int),
                Token::Assign,
                Token::Int(32),
                Token::Semicolon,
            ]
        )
    }

    #[test]
    fn tokenize_assignment_whitespace_termination() {
        assert_eq!(
            tokenize("let x:int=32; "),
            vec![
                Token::Let,
                Token::Var(String::from("x")),
                Token::Colon,
                Token::TypeName(Type::Int),
                Token::Assign,
                Token::Int(32),
                Token::Semicolon,
            ]
        )
    }

    #[test]
    fn tokenize_assignment_min_whitespace() {
        assert_eq!(
            tokenize("let x:int=32;"),
            vec![
                Token::Let,
                Token::Var(String::from("x")),
                Token::Colon,
                Token::TypeName(Type::Int),
                Token::Assign,
                Token::Int(32),
                Token::Semicolon,
            ]
        )
    }

    #[test]
    fn tokenize_assignment_no_whitespace() {
        assert_eq!(
            tokenize("letx:int=32;"),
            vec![
                Token::Var(String::from("letx")),
                Token::Colon,
                Token::TypeName(Type::Int),
                Token::Assign,
                Token::Int(32),
                Token::Semicolon,
            ]
        )
    }

    #[test]
    fn tokenize_str_assignment() {
        assert_eq!(
            tokenize("let x: str = \"Hello World!\";"),
            vec![
                Token::Let,
                Token::Var(String::from("x")),
                Token::Colon,
                Token::TypeName(Type::Str),
                Token::Assign,
                Token::Str(String::from("Hello World!")),
                Token::Semicolon,
            ]
        )
    }

    #[test]
    fn tokenize_bool_assignment() {
        assert_eq!(
            tokenize("let x: bool = true;"),
            vec![
                Token::Let,
                Token::Var(String::from("x")),
                Token::Colon,
                Token::TypeName(Type::Bool),
                Token::Assign,
                Token::Bool(true),
                Token::Semicolon,
            ]
        )
    }

    #[test]
    fn tokenize_void_assignment() {
        assert_eq!(
            tokenize("let x: void = ();"),
            vec![
                Token::Let,
                Token::Var(String::from("x")),
                Token::Colon,
                Token::TypeName(Type::Void),
                Token::Assign,
                Token::LeftParen,
                Token::RightParen,
                Token::Semicolon,
            ]
        )
    }

    #[test]
    fn tokenize_struct_assignment() {
        assert_eq!(
            tokenize(
                "let x: foo = {
					bar: 32,
					baz: \"Hello World\",
				};"
            ),
            vec![
                Token::Let,
                Token::Var(String::from("x")),
                Token::Colon,
                Token::Var(String::from("foo")),
                Token::Assign,
                Token::LeftCurly,
                Token::Var(String::from("bar")),
                Token::Colon,
                Token::Int(32),
                Token::Comma,
                Token::Var(String::from("baz")),
                Token::Colon,
                Token::Str(String::from("Hello World")),
                Token::Comma,
                Token::RightCurly,
                Token::Semicolon,
            ]
        )
    }

    #[test]
    fn tokenize_list_assignment() {
        assert_eq!(
            tokenize("let x: [int] = [32, 17, -5,];"),
            vec![
                Token::Let,
                Token::Var(String::from("x")),
                Token::Colon,
                Token::LeftBrace,
                Token::TypeName(Type::Int),
                Token::RightBrace,
                Token::Assign,
                Token::LeftBrace,
                Token::Int(32),
                Token::Comma,
                Token::Int(17),
                Token::Comma,
                Token::Minus,
                Token::Int(5),
                Token::Comma,
                Token::RightBrace,
                Token::Semicolon,
            ]
        )
    }

    #[test]
    fn tokenize_higher_order_func_assignment() {
        assert_eq!(
            tokenize("let x: (int -> int) = (a) { return 1 + a; };"),
            vec![
                Token::Let,
                Token::Var(String::from("x")),
                Token::Colon,
                Token::LeftParen,
                Token::TypeName(Type::Int),
                Token::Output,
                Token::TypeName(Type::Int),
                Token::RightParen,
                Token::Assign,
                Token::LeftParen,
                Token::Var(String::from("a")),
                Token::RightParen,
                Token::LeftCurly,
                Token::Return,
                Token::Int(1),
                Token::Plus,
                Token::Var(String::from("a")),
                Token::Semicolon,
                Token::RightCurly,
                Token::Semicolon,
            ]
        )
    }

    #[test]
    fn tokenize_struct_definition() {
        assert_eq!(
            tokenize(
                "struct Foo {
						bar: int,
						baz: str,
						qux: bool,
					}"
            ),
            vec![
                Token::Struct,
                Token::Var(String::from("Foo")),
                Token::LeftCurly,
                Token::Var(String::from("bar")),
                Token::Colon,
                Token::TypeName(Type::Int),
                Token::Comma,
                Token::Var(String::from("baz")),
                Token::Colon,
                Token::TypeName(Type::Str),
                Token::Comma,
                Token::Var(String::from("qux")),
                Token::Colon,
                Token::TypeName(Type::Bool),
                Token::Comma,
                Token::RightCurly,
            ]
        )
    }

    #[test]
    fn tokenize_function_definition() {
        assert_eq!(
            tokenize(
                "func bad_adder(a: int, b: int,) -> int {
					let x: int = a;
					let y: int = b;
					let result: int = a + b;
					return result;
				}"
            ),
            vec![
                Token::Function,
                Token::Var(String::from("bad_adder")),
                Token::LeftParen,
                Token::Var(String::from("a")),
                Token::Colon,
                Token::TypeName(Type::Int),
                Token::Comma,
                Token::Var(String::from("b")),
                Token::Colon,
                Token::TypeName(Type::Int),
                Token::Comma,
                Token::RightParen,
                Token::Output,
                Token::TypeName(Type::Int),
                Token::LeftCurly,
                Token::Let,
                Token::Var(String::from("x")),
				Token::Colon,
                Token::TypeName(Type::Int),
                Token::Assign,
                Token::Var(String::from("a")),
                Token::Semicolon,
                Token::Let,
                Token::Var(String::from("y")),
				Token::Colon,
                Token::TypeName(Type::Int),
                Token::Assign,
                Token::Var(String::from("b")),
                Token::Semicolon,
                Token::Let,
                Token::Var(String::from("result")),
				Token::Colon,
                Token::TypeName(Type::Int),
                Token::Assign,
                Token::Var(String::from("a")),
                Token::Plus,
                Token::Var(String::from("b")),
                Token::Semicolon,
                Token::Return,
                Token::Var(String::from("result")),
                Token::Semicolon,
                Token::RightCurly,
            ]
        )
    }

    #[test]
    fn tokenize_function_call() {
        assert_eq!(
            tokenize("foo(x, y);"),
            vec![
                Token::Var(String::from("foo")),
                Token::LeftParen,
                Token::Var(String::from("x")),
                Token::Comma,
                Token::Var(String::from("y")),
                Token::RightParen,
                Token::Semicolon,
            ]
        )
    }

    // ----------- tokenize_str() tests ---------- \\
    #[test]
    fn tokenize_str_one_word() {
        assert_eq!(
            tokenize_str("Hello\""),
            (Token::Str(String::from("Hello")), "")
        )
    }

    #[test]
    fn tokenize_str_two_words() {
        assert_eq!(
            tokenize_str("Hello World\""),
            (Token::Str(String::from("Hello World")), "")
        )
    }

    #[test]
    fn tokenize_str_words_and_remainder() {
        assert_eq!(
            tokenize_str("Hello World\"; let x = 5;"),
            (Token::Str(String::from("Hello World")), "; let x = 5;")
        )
    }

    #[test]
	#[should_panic]
    fn tokenize_str_empty() {
        tokenize_str("");
    }

    #[test]
	#[should_panic]
    fn tokenize_not_a_string() {
        tokenize_str("let x;");
    }

    // ----------- tokenize_int() tests ---------- \\
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

    // ----------------- ordererd chunk tests ------------------ \\
	#[test]
	fn make_chunk_empty() {
		assert_eq!(make_chunk(""), None)
	}

	#[test]
	fn make_chunk_whitespace() {
		assert_eq!(make_chunk(" "), None)
	}

	#[test]
	fn make_chunk_symbol() {
		assert_eq!(
			make_chunk("->()"),
			Some(OrderedChunk {
				alphanumerics: "",
				symbols: "->",
				remainder: "()",
			}))
	}

	#[test]
	fn make_chunk_word() {
		assert_eq!(
			make_chunk("word"),
			Some(OrderedChunk {
				alphanumerics: "word",
				symbols: "",
				remainder: "",
			}))
	}

	#[test]
	fn make_chunk_underscore() {
		assert_eq!(
			make_chunk("_"),
			Some(OrderedChunk {
				alphanumerics: "_",
				symbols: "",
				remainder: "",
			}))
	}

	#[test]
	fn make_chunk_remainder() {
		let chunk = make_chunk(" Hello.World ");
		assert_eq!(
			chunk,
			Some(OrderedChunk {
				alphanumerics: "Hello",
				symbols: ".",
				remainder: "World ",
			}));

		let chunk = make_chunk(chunk.unwrap().remainder);
		assert_eq!(
			chunk,
			Some(OrderedChunk {
				alphanumerics: "World",
				symbols: "",
				remainder: " ",
			}));

		let chunk = make_chunk(chunk.unwrap().remainder);
		assert_eq!(chunk, None);
	}

	#[test]
	fn make_chunk_string() {
		assert_eq!(
			make_chunk("\"Hello World\""),
			Some(OrderedChunk {
				alphanumerics: "",
				symbols: "\"",
				remainder: "Hello World\"",
			}))
	}

    // ----------------- tokenize while tests ------------------ \\
    #[test]
    fn tokenize_while_with_condition(){
        assert_eq!(tokenize("while (x != 5){
        return true; }"),
        vec![
        Token::While,
        Token::LeftParen,
        Token::Var(String::from("x")),
        Token::NotEqual,
        Token::Int(5),
        Token::RightParen,
        Token::LeftCurly,
        Token::Return,
        Token::Bool(true),
        Token::Semicolon,
        Token::RightCurly,]);

        }

    // ----------------- tokenize for loop tests ------------------ \\
    #[test]
    fn tokenize_for_loop(){
        assert_eq!(tokenize("for items in list return items"),
        vec![
        Token::For,
        Token::Var(String::from("items")),
        Token::In,
        Token::Var(String::from("list")),
        Token::Return,
        Token::Var(String::from("items")),]);
    }

    // ----------------- tokenize if else tests ------------------ \\
    #[test]
    fn tokenize_if_else(){
        assert_eq!(tokenize("if(x = 1){
                             y = true;
                         } else y = false;"),
        vec![
        Token::If,
        Token::LeftParen,
        Token::Var(String::from("x")),
        Token::Assign,
        Token::Int(1),
        Token::RightParen,
        Token::LeftCurly,
        Token::Var(String::from("y")),
        Token::Assign,
        Token::Bool(true),
        Token::Semicolon,
        Token::RightCurly,
        Token::Else,
        Token::Var(String::from("y")),
        Token::Assign,
        Token::Bool(false),
        Token::Semicolon]);
    }

}
