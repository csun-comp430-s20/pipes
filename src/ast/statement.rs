use crate::ast::exp::{Exp, Var, ListDef};

pub enum Statement {
    Assignment(Var, Exp),
    Return(Exp),
    If(IfStatement),
    For(ForStatement),
    While(WhileStatement),
}

pub struct IfStatement {
    condition: Exp,
    true_branch: Vec<Statement>,
    false_branch: Option<Vec<Statement>>,
}

pub struct ForStatement {
    iter: Var,
    list: ListDef,
    block: Vec<Statement>,
}

pub struct WhileStatement {
    condition: Exp,
    block: Vec<Statement>,
}
