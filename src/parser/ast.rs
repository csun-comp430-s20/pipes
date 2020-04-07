use super::exp::Exp;
use super::statement::Statement;

pub enum AST {
	NodeStmt(Statement, Box<AST>),
	NodeExp(Exp, Box<AST>),
	Leaf,
}
