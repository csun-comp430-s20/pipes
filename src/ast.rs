use std::collections::HashMap;
use crate::token::Type;

enum Exp {
	Var(Var),
	Int(i32),
	Str(String),
	Bool(bool),

	Negate(Box<Exp>),
	Op(BinaryOp),

	// the hard stuff
	Struct(StructDef),
	List(ListDef),
	HigherOrderFunction(HOF),
}

struct StructDef {
	name: String,
	field: Vec<HashMap<Var, Option<Exp>>>,
}

struct ListDef {
	type: Type,
	item: Vec<Exp>,
}

struct HOF {
	param: Var,
	output: Exp,
}

struct Var {
	name: String,
	type: Type,
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
