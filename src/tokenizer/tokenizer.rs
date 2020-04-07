use super::Token;
use crate::types::Type;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut input = input;

    while input != "" {
        input = skip_whitespace(input);
        if input == "" {
            return tokens;
        }
        let cursor = char::from(input.as_bytes()[0]);

        if cursor == '\"' {
            if let Some((token, remainder)) = tokenize_str(&input[1..]) {
                tokens.push(token);
                input = remainder;
            } else {
                panic!("No closing double-quote: {}", input)
            }
        } else if cursor.is_alphanumeric() || cursor == '_' {
            let (candidate, remainder) = split_first_word(input);
            if let Some(token) = tokenize_int(candidate) {
                tokens.push(token);
                input = remainder;
            } else {
                tokens.push(tokenize_word(candidate));
                input = remainder;
            }
        } else {
            if input.len() >= 2 {
                if let Some(token) = tokenize_symbol(&input[..2]) {
                    tokens.push(token);
                    input = &input[2..];
                    continue;
                }
            }

            if input.len() >= 1 {
                if let Some(token) = tokenize_symbol(&input[..1]) {
                    tokens.push(token);
                    input = &input[1..];
                    continue;
                }
            }

            if input != "" {
                panic!("Failed to parse: {}", input);
            }
        }
    }

    tokens
}

fn tokenize_int(word: &str) -> Option<Token> {
    let word = word.replace("_", "");
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

        "func" => Token::Function,
        "return" => Token::Return,
        "let" => Token::Let,

        "struct" => Token::Struct,
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),

        "int" => Token::TypeName(Type::Int),
        "str" => Token::TypeName(Type::Str),
        "bool" => Token::TypeName(Type::Bool),
        "void" => Token::TypeName(Type::Void),

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
fn split_first_word(s: &str) -> (&str, &str) {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        let c = char::from(item);

        if !c.is_alphanumeric() && c != '_' {
            return (&s[..i], &s[i..]);
        }
    }

    (&s[..], "")
}

//takes a string slice and returns a slice without leading whitespace
fn skip_whitespace(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if !char::from(item).is_whitespace() {
            return &s[i..];
        }
    }
    ""
}

#[cfg(test)]
pub mod tests {
    use super::*;

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
        assert_eq!(
            tokenize("123_this_is_a_var"),
            vec![Token::Var(String::from("123_this_is_a_var"))]
        )
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

    // ----------- skip_whitespace() tests ---------- \\
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

    // ----------- split_first_word() tests ---------- \\
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

    // ----------------- tokenize while tests ------------------ \\
    #[test]
    fn tokenize_while_with_condition() {
        assert_eq!(
            tokenize(
                "while (x != 5){
        return true; }"
            ),
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
                Token::RightCurly,
            ]
        );
    }

    // ----------------- tokenize for loop tests ------------------ \\
    #[test]
    fn tokenize_for_loop() {
        assert_eq!(
            tokenize("for items in list return items"),
            vec![
                Token::For,
                Token::Var(String::from("items")),
                Token::In,
                Token::Var(String::from("list")),
                Token::Return,
                Token::Var(String::from("items")),
            ]
        );
    }

    // ----------------- tokenize if else tests ------------------ \\
    #[test]
    fn tokenize_if_else() {
        assert_eq!(
            tokenize(
                "if(x = 1){
                             y = true;
                         } else y = false;"
            ),
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
                Token::Semicolon
            ]
        );
    }
}
