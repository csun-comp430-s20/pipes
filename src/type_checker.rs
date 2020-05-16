use crate::types::Type;
use crate::ast::*;

pub fn type_check(input: AST) -> Result<AST, &'static str> {
	let input = mut input;
	let output: AST;
	while input != Leaf {
		match input {
			StmtNode(stmt, child) => Err("Statement branch not written"),
			ExpNode(exp, child) => Err("Expression branch not written"),
			Leaf => Ok(output)
		}
	}
}
