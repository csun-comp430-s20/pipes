mod exp;
mod statement;

pub use exp::*;
pub use statement::*;

pub enum AST {
	StmtNode(Statement, Box<AST>),
	ExpNode(Exp, Box<AST>),
	Leaf,
}
