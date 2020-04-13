use crate::types::Type;

pub enum Exp {
    Var(Box<Var>),
    Int(i32),
    Str(String),
    Bool(bool),

	Call(String, Vec<Var>, Box<Var>),
    Negate(Box<Exp>),
    Op(Operation, Box<Exp>, Box<Exp>),

    // the hard stuff
    Struct(String, Vec<Var>),
    List(Type, Vec<Exp>),
    HOF(Box<Var>, Box<Exp>),
}

pub struct Var {
    name: String,
    var_type: Type,
    value: Option<Exp>,
}

pub enum Operation {
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
