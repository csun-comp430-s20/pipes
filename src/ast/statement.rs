use crate::types::Type;
use crate::ast::{Exp, Var};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    Assignment(Var, Exp),
	Function(Function),
    Return(Exp),
    If(IfStatement),
    For(ForStatement),
    While(WhileStatement),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function {
	name: String,
	param: Vec<Var>,
	output: Var,
	block: Vec<Statement>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IfStatement {
    condition: Exp,
    true_branch: Vec<Statement>,
    false_branch: Option<Vec<Statement>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForStatement {
    iter: Var,
    list: (Type, Vec<Exp>),
    block: Vec<Statement>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WhileStatement {
    condition: Exp,
    block: Vec<Statement>,
}
