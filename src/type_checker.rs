// use crate::types::Type;
use crate::ast::*;

pub fn type_check(input: AST) -> Result<AST, &'static str> {
	let mut input = input;
	let mut output = AST::Leaf;
	loop {
		input = match &input {
			AST::StmtNode(stmt, box child) => {
				output = AST::StmtNode(
					type_check_statement(stmt)?,
					Box::new(output));
				(*child).clone()
			},
			AST::ExpNode(exp, box child) => {
				output = AST::ExpNode(
					type_check_expression(exp)?,
					Box::new(output));
				(*child).clone()
			},
			AST::Leaf => return Ok(output),
		};
	}
}

fn type_check_expression(_stmt: &Exp) -> Result<Exp, &'static str> {
	Err("Type Check statement not written")
}

fn type_check_statement(_stmt: &Statement) -> Result<Statement, &'static str> {
	Err("Type Check statement not written")
}

#[cfg(test)]
pub mod tests {
	use super::*;
	#[test]
	fn type_check_leaf() {
		assert_eq!(Ok(AST::Leaf), type_check(AST::Leaf))
	}
}
