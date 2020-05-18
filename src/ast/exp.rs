use crate::types::Type;

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Var {
    name: String,
    var_type: Type,
    value: Option<Exp>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
