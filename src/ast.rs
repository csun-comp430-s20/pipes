mod exp;
mod statement;

pub use exp::*;
pub use statement::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AST {
	StmtNode(Statement, Box<AST>),
	ExpNode(Exp, Box<AST>),
	Leaf,
}
