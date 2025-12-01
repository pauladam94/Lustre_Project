use crate::{
    checker::test::ok_check,
    parser::{ast::ast, span::LSpan},
};
use colored::Colorize;

pub fn ok_interpretation(input: &str) {
    ok_check(input);
    let (_, build_ast) = ast(LSpan::new(input)).unwrap();

    let (const_ast, _) = build_ast.propagate_const();
    println!("{}\n{}", ">> Propagate Constant :".blue(), const_ast);

    for node in const_ast.nodes.iter() {
        if node.is_test() && !node.is_only_true_equations() {
            assert!(false);
        }
    }
}

pub fn error_interpretation(input: &str) {
    ok_check(input);
    let (_, build_ast) = ast(LSpan::new(input)).unwrap();
    let (const_ast, _) = build_ast.propagate_const();
    println!("{}\n{}", ">> Propagate Constant :".blue(), const_ast);

    for node in const_ast.nodes.iter() {
        if node.is_test() && node.is_only_true_equations() {
            assert!(false);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::interpreter::test::{error_interpretation, ok_interpretation};

    #[test]
    fn double_ok() {
        ok_interpretation(
            "
node incr(i : int) returns (o : int);
let
    o = i + 1;
tel
#[test]
node test() returns (z : bool);
let
    lhs = incr([1, 2, 3, 4, 5]);
    rhs = [2, 3, 4, 5, 6];
    z = lhs == rhs;
tel
",
        );
    }
    #[test]
    fn double_error() {
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
	rhs = [2, 4, -7];
	b = lhs == rhs;
tel",
        );
    }
    #[test]
    fn triple_ok() {
        ok_interpretation(
            "
node triple_and_incr(i : int) returns (o : int);
let
    o = 1 + i * 3;
tel
#[test]
node test() returns (z : bool);
let
    lhs = triple_and_incr([1, 2, 3]);
    rhs = [4, 7, 10];
    z = lhs == rhs;
tel
",
        );
    }
    #[test]
    fn triple_error() {
        error_interpretation(
            "
node triple_and_incr(i : int) returns (o : int);
let
    o = 1 + i * 3;
tel
#[test]
node test() returns (z : bool);
let
    lhs = triple_and_incr([1, 2, 3]);
    rhs = [4, 7, 9];
    z = lhs == rhs;
tel
",
        );
    }

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
    fn fibonacci_12_ok() {
        error_interpretation(
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
	z = fibo([(), (), (), (), ()]) == [3, 3, 5, 8, 13];
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
    fn cst_ok() {
        ok_interpretation(
            "
node cst() returns (z: int);
let
    z = 12 fby z;
tel
#[test]
node test() returns (z: bool);
let
    lhs = cst([(), (), ()]);
    rhs = [12, 12, 12];
    z = lhs == rhs;
tel
            ",
        );
    }
    #[test]
    fn id_error() {
        error_interpretation(
            "
node cst() returns (z: int);
let
    z = 12 fby z;
tel
#[test]
node test() returns (z: bool);
let
    lhs = cst([(), (), ()]);
    rhs = [12, 13, 12];
    z = lhs == rhs;
tel
            ",
        );
    }
    #[test]
    fn id_two_var_ok() {
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

    #[test]
    fn time_0_ok() {
        ok_interpretation(
            "
node time() returns (z : int);
let
    z = 0 fby (z + 1);
tel

#[test]
node test() returns (z: bool);
let
    lhs = [0, 1, 2, 3, 4, 5];
    rhs = time([(), (), (), (), (), ()]);
    z = lhs == rhs;
tel
    ",
        )
    }
    #[test]
    fn time_1_ok() {
        ok_interpretation(
            "
node time() returns (z : int);
let
    z = (0 fby z) + 1;
tel

#[test]
node test() returns (z: bool);
let
    lhs = [1, 2, 3, 4, 5];
    rhs = time([(), (), (), (), ()]);
    z = lhs == rhs;
tel
    ",
        )
    }
    #[test]
    fn time_2_ok() {
        ok_interpretation(
            "
node time() returns (z : int);
let
    z = 0 fby (z + 1);
tel

node time_call() returns (z : int);
let
    z = time();
tel

#[test]
node test() returns (z: bool);
let
    lhs = [0, 1, 2, 3, 4, 5];
    rhs = time_call([(), (), (), (), (), ()]);
    z = lhs == rhs;
tel
    ",
        )
    }

    #[test]
    fn time_3_ok() {
        ok_interpretation(
            "
node time() returns (z : int);
let
    z = 0 fby z + 1;
tel

node time_call() returns (z : int);
let
    z = time();
tel

#[test]
node test() returns (z: bool);
let
    lhs = [1, 2, 3, 4, 5, 6];
    rhs = time_call([(), (), (), (), (), ()]);
    z = lhs == rhs;
tel
    ",
        )
    }
}

/*

let f x y z = ...

f    1      2      3 =>  (((f 1)      2)    3)
f : int -> int -> int =>    (int -> (int -> (int)))




*/
