use colored::Colorize;

use crate::parser::{ast::ast, span::LSpan, test::ok_parse};

pub fn ok_check(input: &str) {
    ok_parse(input);
    let (_, build_ast) = ast(LSpan::new(input)).unwrap();

    let diags = build_ast.check();
    if !diags.is_empty() {
        println!(
            "{} : diagnostics should be empty\n>>{} = {:#?}",
            ">> ERROR".red(),
            "DIAGS".blue(),
            diags
        );
        assert!(false);
    }
}

pub fn error_check(input: &str) {
    ok_parse(input);
    let (_, build_ast) = ast(LSpan::new(input)).unwrap();

    let diags = build_ast.check();
    if diags.is_empty() {
        println!("{}: error expected but no diagnostics", ">> ERROR".red());
        assert!(false);
    }
}

#[cfg(test)]
mod tests {
    use crate::checker::test::{error_check, ok_check};

    #[test]
    fn test1() {
        ok_check(
            "
node f() returns();
let
tel
",
        )
    }
    #[test]
    fn test2() {
        ok_check(
            "
node f(x: int) returns(z: int);
let
    z = x;
tel
",
        )
    }
    #[test]
    fn test3() {
        error_check(
            "
node f(x: int, x : int) returns(z: int);
let
    z = x;
tel
",
        )
    }

    #[test]
    fn test4() {
        error_check(
            "
node f(x: int) returns(z: int);
let
    z = 2 + a;
tel
",
        )
    }
    #[test]
    fn test5() {
        error_check(
            "
node f(x: int) returns(z: int);
let
    z = 2 + true;
tel
",
        )
    }
    #[test]
    fn test6() {
        ok_check(
            "
node f(x: int) returns(z: int);
let
    z = x + 10;
tel
",
        )
    }
    #[test]
    fn test7() {
        ok_check(
            "
node f(x: int) returns(z: int, y: bool);
let
    a = x + 5;
    z = a + x + 10;
    y = true;
tel
",
        )
    }
}
