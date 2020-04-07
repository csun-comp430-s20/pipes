#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Void,
    Int,
    Str,
    Bool,
    Struct(String),
    HigherOrderFunction(Box<Type>, Box<Type>),
    ListInt,
    ListStr,
    ListBool,
    ListStruct(String),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    If,   // if
    Elif, // elif
    Else, // else

    For,   // for
    In,    // in
    While, // while

    Return, // return
    Output, // ->
    Let,    // let
    Assign, // =

    TypeName(Type),
    Struct,      // struct
    Function,    // func
    Int(i32),    // int
    Bool(bool),  // bool
    Str(String), // str
    Var(String),
    Comment(String), // #

    LeftCurly,  // {
    LeftBrace,  // [
    LeftParen,  // (
    RightCurly, // }
    RightBrace, // ]
    RightParen, // )

    Dot,       // .
    Comma,     // ,
    Colon,     // :
    Semicolon, // ;

    Minus,    // -
    Plus,     // +
    Divide,   // /
    Multiply, // *
    Modulo,   // %

    And, // &&
    Or,  // ||
    Not, // !

    GreaterThan,  // >
    LessThan,     // <
    GreaterEqual, // >=
    LessEqual,    // <=
    Equal,        // ==
    NotEqual,     // !=
}

enum Statement {
	Assignment(Var, Exp),
	Return(Exp),
	If(IfStatement),
	For(ForStatement),
	While(WhileStatement),
}

struct IfStatement {
	condition: Exp,
	true_branch: Vec<Statement>,
	false_branch: Option<Vec<Statement>>,
}

struct ForStatement {
	iter: Var,
	list: ListDef,
	block: Vec<Statement>,
}

struct WhileStatement {
	condition: Exp,
	block: Vec<Statement>,
}

enum Exp {
	Var(Box<Var>),
	Int(i32),
	Str(String),
	Bool(bool),

	Negate(Box<Exp>),
	Op(Box<Op>),

	// the hard stuff
	Struct(StructDef),
	List(ListDef),
	HigherOrderFunction(Box<HOF>),
}

struct StructDef {
	name: String,
	field: Vec<Var>,
}

struct ListDef {
	list_type: Type,
	item: Vec<Exp>,
}

struct HOF {
	param: Var,
	output: Exp,
}

struct Var {
	name: String,
	var_type: Type,
	value: Option<Exp>,
}

struct Op {
	op: Operation,
	left: Exp,
	right: Exp,
}
enum Operation {
	Add,
	Subtract,
	Divide,
	Multiply,
	Modulo,

	And,
	Or,

    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
    Equal,
    NotEqual,
}
