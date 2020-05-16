use crate::types::Type;
use crate::ast::*;

pub fn type_check(input: AST) -> Result<AST, &'static str> {
	let mut input = input;
	let mut output = AST::Leaf;
	loop {
		match &input {
			AST::StmtNode(stmt, child) => Err("Statement branch not written"),
			AST::ExpNode(exp, child) => Err("Expression branch not written"),
			AST::Leaf => Ok(AST::Leaf),
		};
	};
}

#[cfg(test)]
pub mod tests {
	use super::*;
	#[test]
	fn type_check_leaf() {
		assert_eq!(Ok(AST::Leaf), type_check(AST::Leaf))
	}
}
