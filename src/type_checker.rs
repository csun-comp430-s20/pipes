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
    use crate::types::Type;
    use crate::tokenizer::tokenizer::*;
    use crate::parser::*;
	
	#[test]
	fn type_check_leaf() {
		assert_eq!(Ok(AST::Leaf), type_check(AST::Leaf))
	}

	//==========================================
	// some test cases

	#[test]
	//  let x:int = 1;	
	fn type_check_int_var(){
		let token = tokenize("let x: int = 1;");
		let parsed = parse(token);
		assert_eq!(type_check(parsed), Type::Int);
	}

	#[test]
	//  5+1;	
	fn type_check_int_add_operation(){
		let token = tokenize("5+1;");
		let parsed = parse(token);
		assert_eq!(type_check(parsed), Type::Int);
	}
	#[test]
	// 4-2
	fn type_check_int_substract_operation(){
		let token = tokenize("4-2;");
		let parsed = parse(token);
		assert_eq!(type_check(parsed), Type::Int);
	}

	#[test]
	fn type_check_int_modulo_operation(){
		let token = tokenize("10%2;");
		let parsed = parse(token);
		assert_eq!(type_check(parsed), Type::Int);
	}
	
	#[test]
	fn type_check_int_divide_operation(){
		let token = tokenize("4/2;");
		let parsed = parse(token);
		assert_eq!(type_check(parsed), Type::Int);
	}

	#[test]
	fn type_check_int_multiply_operation(){
		let token = tokenize("3*3;");
		let parsed = parse(token);
		assert_eq!(type_check(parsed), Type::Int);
	}
	
	#[test]
	// let x: int = 1 + 2;
	fn type_check_int_var_add(){
		let token = tokenize("let x: int = 1 + 2;");
		let parsed = parse(token);
		assert_eq!(type_check(parsed), Type::Int);
	}

	#[test]
	// let x: int = 3 - 2;
	fn type_check_int_var_substract(){
		let token = tokenize("let x: int = 3 - 2;");
		let parsed = parse(token);
		assert_eq!(type_check(parsed), Type::Int);
	}
	#[test]
	// let x: int = 9 / 3;
	fn type_check_int_var_divide(){
		let token = tokenize("let x: int = 9 / 3;");
		let parsed = parse(token);
		assert_eq!(type_check(parsed), Type::Int);
	}

	#[test]
	// let x: int = 4 % 2;
	fn type_check_int_var_modulo(){
		let token = tokenize("let x: int = 4 % 2;");
		let parsed = parse(token);
		assert_eq!(type_check(parsed), Type::Int);
	}

	#[test]
	// let x: int = 2 * 3 + 8 / 2;
	fn type_check_int_var_multiple_operations(){
		let token = tokenize("let x: int = 2 * 3 + 8 / 2;");
		let parsed = parse(token);
		assert_eq!(type_check(parsed), Type::Int);
	}

	#[test]
	// for x in(9)
	fn type_check_int_for_loop(){
		let token = tokenize("for x in(9);");
		let parsed = parse(token);
		assert_eq!(type_check(parsed), Type::Int);
	}

	#[test]
	// let s: str = "Hi!"
	fn type_check_str_assign(){
		let token = tokenize("let s: str = Hi!;");
		let parsed = parse(token);
		assert_eq!(type_check(parsed), Type::Str);
	}

//==================Boolean Test Cases=====================
    #[test]
    fn type_check_bool() {
        let token = tokenize("true");
        let parsed = parse(token);
        assert_eq!(type_check(parsed), Type::Bool);
    }

    #[test]
    fn type_check_let_bool() {
        let token = tokenize("let x: bool = false");
        let parsed = parse(token);
        assert_eq!(type_check(parsed), Type::Bool);
    }

    #[test]
    fn type_check_not_equal_bool() {
        let token = tokenize("1 != 0");
        let parsed = parse(token);
        assert_eq!(type_check(parsed), Type::Bool);
    }

    #[test]
    fn type_check_equal_bool() {
        let token = tokenize("0 == 0");
        let parsed = parse(token);
        assert_eq!(type_check(parsed), Type::Bool);
    }

    #[test]
    fn type_check_and_bool() {
        let token = tokenize("x && y");
        let parsed = parse(token);
        assert_eq!(type_check(parsed), Type::Bool);
    }

    #[test]
    fn type_check_or_bool() {
        let token = tokenize("x || y");
        let parsed = parse(token);
        assert_eq!(type_check(parsed), Type::Bool);
    }

    #[test]
    fn type_check_if_and_bool() {
        let token = tokenize("if(x == y && x > 10)");
        let parsed = parse(token);
        assert_eq!(type_check(parsed), Type::Bool);
    }

    #[test]
    fn type_check_if_or_bool() {
        let token = tokenize("if(x == y || z < 5)");
        let parsed = parse(token);
        assert_eq!(type_check(parsed), Type::Bool);
    }

    #[test]
    fn type_check_return_bool() {
        let token = tokenize("return true");
        let parsed = parse(token);
        assert_eq!(type_check(parsed), Type::Bool);
    }
}
