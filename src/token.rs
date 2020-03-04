#[derive(Debug, Eq, PartialEq)]
pub enum Types {
	Void,
	Int,
	Str,
	Bool,
	Struct(String),
	HOF,
	List_Int,
	List_Str,
	List_Bool,
	List_Struct(String),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
	If,              // if
	Elif,            // elif
	Else,            // else

	For,             // for
	In,              // in
	While,           // while

	Return,          // return
	Output,          // ->
	Let,             // let
	Assign,          // =

	Void,
	Type(String),
	Struct(String),  // struct
	Int(i32),        // int
	Bool(bool),      // bool
	Str(String),     // str
	Var(String),
	Comment(String), // #

	LeftCurly,       // {
	LeftBrace,       // [
	LeftParen,       // (
	RightCurly,      // }
	RightBrace,      // ]
	RightParen,      // )

	Dot,             // .
	Comma,           // ,
	Colon,           // :
	Semicolon,       // ;

	Minus,           // -
	Plus,            // +
	Divide,          // /
	Multiply,        // *
	Modulo,          // %

	And,             // &&
	Or,              // ||
	Not,             // !

	GreaterThan,     // >
	LessThan,        // <
	GreaterEqual,    // >=
	LessEqual,       // <=
	Equal,           // ==
	NotEqual,        // !=
}
