use crate::ast::*;
use crate::tokenizer::Token;

pub fn parse(input: Vec<Token>) -> Result<AST, &'static str> {
	try_parse_exp(input.clone());
	try_parse_stmt(input.clone());
	Err("Parser not written")
}

fn try_parse_stmt(_input: Vec<Token>) -> Option<Statement> {
	// try all of em, all of our statements will also require parsing exp
	None
}

fn try_parse_exp(_input: Vec<Token>) -> Option<Exp> {
	// try primary (var, int, str, bool)
	// try operation
	// try data structure
	None
}

// #[cfg(test)]
// pub mod tests {
//     use super::*;
//     use crate::tokenizer::token::Token;
// 
//     //Operation tests
//     #[test]
//     fn parser_add_two_ints() {
//         // 1+2
//         let tokens = tokenize("1+2");
//         assert_eq!(
//             parse(tokens),
//             ast(OP(Operation::Add, left(Exp::Int(1)), right(Exp::Int(2))))
//         )
//     }
// 
//     #[test]
//     fn parser_substract_two_ints() {
//         let tokens = tokenize("4-2");
//         assert_eq!(
//             parse(tokens),
//             ast(OP(
//                 Operation::Subtract,
//                 left(Exp::Int(4)),
//                 right(Exp::Int(2))
//             ))
//         )
//     }
// 
//     #[test]
//     fn parser_multiply_two_ints() {
//         let tokens = tokenize("2*5");
//         assert_eq!(
//             parse(tokens),
//             ast(OP(
//                 Operation::Multiply,
//                 left(Exp::Int(2)),
//                 right(Exp::Int(5))
//             ))
//         )
//     }
// 
//     #[test]
//     fn parser_divide_two_ints() {
//         let tokens = tokenize("2/2");
//         assert_eq!(
//             parse(tokens),
//             ast(OP(Operation::Divide, left(Exp::Int(2)), right(Exp::Int(2))))
//         )
//     }
// 
//     #[test]
//     fn parser_modulo_two_ints() {
//         let tokens = tokenize("4 % 2");
//         assert_eq!(
//             parse(tokens),
//             ast(OP(Operation::Modulo, left(Exp::Int(4)), right(Exp::Int(2))))
//         )
//     }
// 
//     #[test]
//     fn parser_great_than_two_ints() {
//         let tokens = tokenize("3 > 2");
//         assert_eq!(
//             parse(tokens),
//             ast(OP(
//                 Operation::GreaterThan,
//                 left(Exp::Int(3)),
//                 right(Exp::Int(2))
//             ))
//         )
//     }
// 
//     #[test]
//     fn parser_less_than_two_ints() {
//         let tokens = tokenize("3 < 5");
//         assert_eq!(
//             parse(tokens),
//             ast(OP(
//                 Operation::LessThan,
//                 left(Exp::Int(3)),
//                 right(Exp::Int(5))
//             ))
//         )
//     }
// 
//     #[test]
//     fn parser_great_equal_two_ints() {
//         let tokens = tokenize("3 >= 2");
//         assert_eq!(
//             parse(tokens),
//             ast(OP(
//                 Operation::GreaterEqual,
//                 left(Exp::Int(3)),
//                 right(Exp::Int(2))
//             ))
//         )
//     }
//     #[test]
//     fn parser_less_equal_two_ints() {
//         let tokens = tokenize("3 <= 4");
//         assert_eq!(
//             parse(tokens),
//             ast(OP(
//                 Operation::LessEqual,
//                 left(Exp::Int(3)),
//                 right(Exp::Int(4))
//             ))
//         )
//     }
// 
//     #[test]
//     fn parser_equal_two_ints() {
//         let tokens = tokenize("3 == 3");
//         assert_eq!(
//             parse(tokens),
//             ast(OP(Operation::Equal, left(Exp::Int(3)), right(Exp::Int(3))))
//         )
//     }
// 
//     #[test]
//     fn parser_not_equal_two_ints() {
//         let tokens = tokenize("3 != 2");
//         assert_eq!(
//             parse(tokens),
//             ast(OP(
//                 Operation::NotEqual,
//                 left(Exp::Int(3)),
//                 right(Exp::Int(2))
//             ))
//         )
//     }
// 
//     #[test]
//     // let x: Int = 1;
//     fn parser_int_variable_assignment() {
//         let tokens = tokenize("let x: Int = 1;");
//         assert_eq!(parse(tokens),
// 			ast(Statement::Assignment(Exp::Var(name::String::from("x"), type: Type::Int, value::None), 
// 			Exp::Int(1))))
//     }
// 
//     //Integer assignment with order of operations
//     #[test]
//     fn parse_int_assign_1() {
//         let tokens = tokenize("let x: int = 1 + 2;");
//         assert_eq!(
//             parse(tokens),
//             ast(Assignment(
//                 Exp::Var(name::String::from("x"), Type::Int, value::None),
//                 Op(Operation::Add, left(Exp::Int(1)), right(Exp::Int(2)))
//             ))
//         );
//     }
// 
//     #[test]
//     fn parse_int_assign_2() {
//         let tokens = tokenize("let x: int = 1 - 2;");
//         assert_eq!(
//             parse(tokens),
//             ast(Assignment(
//                 Exp::Var(name::String::from("x"), Type::Int, value::None),
//                 Op(Operation::Minus, left(Exp::Int(1)), right(Exp::Int(2)))
//             ))
//         );
//     }
// 
//     #[test]
//     fn parse_int_assign_3() {
//         let tokens = tokenize("let x: int = 4 * 5;");
//         assert_eq!(
//             parse(tokens),
//             ast(Assignment(
//                 Exp::Var(name::String::from("x"), Type::Int, value::None),
//                 Op(Operation::Multiply, left(Exp::Int(4)), right(Exp::Int(5)))
//             ))
//         );
//     }
// 
//     #[test]
//     fn parse_int_assign_4() {
//         let tokens = tokenize("let x: int = 9 / 3;");
//         assert_eq!(
//             parse(tokens),
//             ast(Assignment(
//                 Exp::Var(name::String::from("x"), Type::Int, value::None),
//                 Op(Operation::Divide, left(Exp::Int(9)), right(Exp::Int(3)))
//             ))
//         );
//     }
// 
//     #[test]
//     fn parse_int_assign_5() {
//         let tokens = tokenize("let x: int = 1 + 2 * 3;");
//         assert_eq!(
//             parse(tokens),
//             ast(Assignment(
//                 Exp::Var(name::String::from("x"), Type::Int, value::None),
//                 Op(
//                     Operation::Add,
//                     left(Exp::Int(1)),
//                     right(OP(
//                         Operation::Multiply,
//                         left(Exp::Int(2)),
//                         right(Exp::Int(3))
//                     ))
//                 )
//             ))
//         );
//     }
// 
//     #[test]
//     fn parse_int_assign_6() {
//         let tokens = tokenize("let x: int = 2 * 3 + 8 / 2;");
//         assert_eq!(
//             parse(tokens),
//             ast(Assignment(
//                 Exp::Var(name::String::from("x"), Type::Int, value::None),
//                 Op(
//                     Operation::Operation::Add,
//                     left(Op(
//                         Operation::Multiply,
//                         left(Exp::Int(2)),
//                         right(Exp::Int(3))
//                     )),
//                     right(Op(Operation::Divide, left(Exp::Int(8)), right(Exp::Int(2))))
//                 )
//             ))
//         );
//     }
// 
//     //String assignments
//     #[test]
//     fn parse_str_assign() {
//         let tokens = tokenize("let x: str = \"Hello World!\";");
//         assert_eq!(
//             parse(tokens),
//             ast(Assignment(
//                 Exp::Var(name::String::from("x"), Type::Str, value::None),
//                 Str("Hello World!")
//             ))
//         );
//     }
// 
//     //While loop tests
//     #[test]
//     fn parse_while() {
//         let tokens = tokenize("while (x <= 9) { return true; }");
//         assert_eq!(
//             parse(tokens),
//             ast(
//                 While(Op(
//                     Operation::LessEqual,
//                     left(Exp::Var(x)),
//                     right(Exp::Int(9))
//                 )),
//                 Return(Exp::Bool(true))
//             )
//         );
//     }
// 
//     //For loop tests
//     #[test]
//     fn parse_for_loop() {
//         let tokens = tokenize("for x in(9)");
//         assert_eq!(
//             parse(tokens),
//             ast(For(
//                 iter(Var::Var(name::String::from("x"), Type::Int, value::Int(0))),
//                 list(ListDef::Type(Type::Int))
//             ))
//         );
//     }
// 
//     //If Else tests
//     #[test]
//     fn parse_if() {
//         let tokens = tokenize("if (x < 5) { return true; }");
//         assert_eq!(
//             parse(tokens),
//             ast(If(
//                 condition(Exp::Op(
//                     Operation::LessThan,
//                     left(Exp::Var(x)),
//                     right(Exp::Int(5))
//                 )),
//                 true_branch(Return(Exp::Bool(true)))
//             ))
//         );
//     }
// 
//     #[test]
//     fn parse_if_else() {
//         let tokens = tokenize("if (x < 5) { return true; } else { return false; }");
//         assert_eq!(
//             parse(tokens),
//             ast(If(
//                 condition(Exp::Op(
//                     Operation::LessThan,
//                     left(Exp::Var(x)),
//                     right(Exp::Int(5))
//                 )),
//                 true_branch(Return(Exp::Bool(true))),
//                 flase_branch(Return(Exp::Bool(false)))
//             ))
//         );
//     }
// }
