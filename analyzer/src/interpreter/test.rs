use crate::{
    checker::test::ok_check,
    parser::{ast::ast, expression::Expr, literal::Value, span::LSpan},
};
use colored::Colorize;

pub fn ok_interpretation(input: &str) {
    ok_check(input);
    let (_, build_ast) = ast(LSpan::new(input)).unwrap();

    let (const_ast, _) = build_ast.propagate_const();
    println!("{}\n{}", ">> Propagate Constant :".blue(), const_ast);

    for node in const_ast.nodes.iter() {
        if node.tag.is_some() {
            let equations = &node.let_bindings;

            // Verify it is a test of type (unit -> bool)
            // More than One equation
            if equations.len() != 1 // has one equation
                || equations[0].0.fragment() != node.outputs[0].0.fragment() // une seule equation
                || equations[0].1 != Expr::Lit(Value::Bool(true))
            {
                assert!(false);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::interpreter::test::ok_interpretation;

    #[test]

    fn integer_operation_propagate_no_test_ok() {
        ok_interpretation(
            "
node f() returns (x : int);
let
    z = 12 / 2;
    x = (1 + 12) * 23 + z + 23 - 6; 
tel",
        );
    }
    #[test]
    fn fibonacci_1_ok() {
        ok_interpretation(
            "
node fibo() returns (x : int);
let
    x = 1 fby x + (1 fby (1 fby x)); 
tel

#[test]
node verify_5_first_value() returns (z: bool);
let
    z = fibo([(), (), (), (), ()]) == [2, 3, 5, 8, 13];        
tel
        ",
        );
    }
    #[test]
    fn fibonacci_11_ok() {
        ok_interpretation(
            "
node fibo() returns (x : int);
let
	x_1 = 1 -> pre x;
	x_0 = 1 -> pre x_1;
	x = x_0 + x_1;
tel

#[test]
node verify_5_first_value() returns (z : bool);
let
	z = fibo([(), (), (), (), ()]) == [2, 3, 5, 8, 13];
tel",
        );
    }
    #[test]
    fn fibonacci_2_ok() {
        ok_interpretation(
            "
node fibo() returns (x : int);
let
    x_1 = 1 fby x;
    x_0 = 1 fby x_1; 
    x = x_0 + x_1; 
tel

#[test]
node verify_5_first_value() returns (z: bool);
let
    z = fibo([(), (), (), (), ()]) == [2, 3, 5, 8, 13];        
tel
        ",
        );
    }
    #[test]
    fn fibonacci_3_ok() {
        ok_interpretation(
            "
node fibo() returns (x : int);
let
    x_1 = 1 fby x;
    x_0 = 1 fby x_1; 
    x = x_0 + x_1; 
tel

#[test]
node verify_5_first_value() returns (z: bool);
let
    lhs =  fibo([(), (), (), (), ()]);
    rhs = [2, 3, 5, 8, 13];
    z = lhs == rhs;        
tel
        ",
        );
    }

    #[test]
    fn has_been_true_ok() {
        ok_interpretation(
            "
node has_been_true(a : bool) returns (z: bool);
let
    z = (false fby z) or a;
tel

#[test]
node test() returns (z: bool);
let
    z = has_been_true([false, true, true, false, false, true]) ==
                      [false, true, true, true, true, true];        
tel
        ",
        );
    }
    #[test]
    fn double_ok() {
        ok_interpretation(
            "
node double(x : int) returns (z : int);
let
	z = x + x;
tel

#[test]
node test() returns (b : bool);
let
	lhs = double([1, 2, 3]);
	rhs = [2, 4, 6];
	b = lhs == rhs;
tel",
        );
    }

    #[test]
    fn id_ok() {
        ok_interpretation(
            "
node id(x, y: int) returns (a, b: int);
let
    a = x;
    b = y;
tel
#[test]
node test() returns (b : bool);
let
    lhs = ([1, 2, 3, 4], [2, 4, 6, 8]);
    rhs = id([1, 2, 3, 4], [2, 4, 6, 8]);
    b = lhs == rhs;
tel
            ",
        );
    }
    #[test]
    fn swith_ok() {
        ok_interpretation(
            "
node switch(x, y : int) returns (a, b : int);
let
    a = y;
    b = x;
tel

#[test]
node test() returns (z: bool);
let
    lhs = ([2, 4, 6, 8], [1, 2, 3, 4]);
    rhs = id([1, 2, 3, 4], [2, 4, 6, 8]);
    b = lhs == rhs;
tel
",
        );
    }
}
