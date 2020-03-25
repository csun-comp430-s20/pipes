use crate::token::Type;

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
	field: Vec<Var>,
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
