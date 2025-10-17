use crate::parser::{
    ast::ast,
    double_visitor::{DoubleTogetherVisitor, ShallowEq},
    span::LSpan,
};
use colored::Colorize;

pub fn error_parse(input: &str) {
    let span = LSpan::new(input);

    match ast(span) {
        Ok((_, res)) => {
            println!("{}\n{input}", ">> input : ".blue());
            println!("{}\n{res}", ">> result : ".red());
            assert!(false);
        }
        _ => (),
    }
}

// #[allow(unused)]
pub fn ok_parse(input: &str) {
    struct Test {
        name: &'static str,
        passed: bool,
    }
    println!("\n{}\n{input}", ">> in :".blue());

    let mut tests = vec![
        Test {
            name: "in | parse | display == in",
            passed: false,
        },
        Test {
            name: "in | parse_rest == \"\"",
            passed: false,
        },
        Test {
            name: "in | parse | display | parse shallow== in | parse",
            passed: false,
        },
        Test {
            name: "in | parse | display | parse | display == in | parse | display",
            passed: false,
        },
    ];
    let span = LSpan::new(input);

    match ast(span) {
        Err(err) => {
            println!("\n{} parsing of \"in\": {}", "FAILED".red(), err);
        }
        Ok((in_rest, in_parse)) => {
            let in_parse_display = format!("{in_parse}");
            tests[0].passed = in_parse_display == input;

            println!("{}\n{in_parse}", ">> in | parse :".blue());
            println!("{}\n{in_rest}", ">> in | parse_rest".purple());

            tests[1].passed = *in_rest.fragment() == "";

            let span = LSpan::new(&in_parse_display);

            match ast(span) {
                Err(err) => {
                    println!(
                        "\n{} parsing of \"in | parse | display\": {}",
                        "FAILED".red(),
                        err
                    );
                }
                Ok((_, in_parse_display_parse)) => {
                    let mut shallow_eq = ShallowEq::default();
                    shallow_eq.walk(&in_parse, &in_parse_display_parse);
                    tests[2].passed = shallow_eq.is_eq();

                    // in_parse_display_parse == in_parse;
                    let in_parse_display_parse_display =
                        format!("{in_parse_display_parse}");
                    tests[3].passed =
                        in_parse_display_parse_display == in_parse_display;

                    println!(
                        "{}\n{in_parse_display_parse}",
                        ">> in | parse | display | parse :".blue()
                    );
                }
            }
        }
    }

    let mut one_test_failed = false;
    for (i, test) in tests.iter().enumerate() {
        if i == 0 {
            // skip first test because I don't care
            continue;
        }
        if test.passed {
            print!("{}", "PASSED: ".green());
        } else {
            one_test_failed = true;
            print!("{}", "FAILED: ".red());
        }
        println!("{}", test.name);
    }

    if one_test_failed {
        assert!(false)
    }
}

#[cfg(test)]
mod test {
    use crate::parser::test::{error_parse, ok_parse};

    #[test]
    fn t1() {
        ok_parse(
            "
node f() returns ();
let
tel;
",
        );
    }
    #[test]
    fn t111() {
        ok_parse("");
    }
    #[test]
    fn t15() {
        ok_parse(
            "
node f() returns ( b : int );
let
tel;
",
        );
    }
    #[test]
    fn t11() {
        ok_parse(
            "
node f (a: int) returns ( ) ;
let

tel ;
",
        );
    }
    #[test]
    fn t13() {
        ok_parse(
            "
node f (a: int) returns ( ) ;
let
    x = y;
tel ;
",
        );
    }
    #[test]
    fn t14() {
        ok_parse(
            "
node f (a: int) returns ( ) ;
let
    x = (y + x);
tel ;
",
        );
    }
    #[test]
    fn t12() {
        ok_parse(
            "
node f (a: int) returns (b:int) ;
let

tel ;
",
        );
    }

    #[test]
    fn t2() {
        ok_parse(
            "
node f(i : int) returns (f : float);
let
tel;
",
        );
    }

    #[test]
    fn t3() {
        ok_parse(
            "
node f(i : int) returns (f : float);
let
    a = x * x;
    f = 2;
tel;
",
        );
    }

    #[test]
    fn t4() {
        ok_parse(
            "
node f(i : int) returns (f : float);
let
    a = x * x;
    f = 2;
tel;

node other_function(i: int, a : int) returns (f: float);
let b   = x + x;
c = d;
tel;
",
        );
    }

    #[test]
    fn t5() {
        ok_parse(
            "
node f(i : int) returns (f : float);
let
    a = x * x;
    f = 2;
tel;

node other_function(i: int, a : int) returns (f: float);
let b   = x + x;
c = d;
tel;

node while(i : int) returns (g : int, t : int);
let
    a = x * x;
    f = 2 + 3;
    t = x + x * y + 2;
tel;
",
        );
    }

    #[test]
    fn t6() {
        ok_parse(
            "
node f(i1 : int, i2 : int, i3 : float, i4 : char) returns (f : float);
let
    a = x * x;
    f = 2;
tel;

node otherfunction(i: int, a : int) returns (f: float);
let b   = x + x;
c = d;
tel;

node while(i : int) returns (g : int, t : int);
let
    a = x * x;
    f = 2 + 3;
    t = x + x * y + 2;
tel;
",
        );
    }
    #[test]
    fn t7() {
        ok_parse(
            "
node f(i1 : int, i2 : int, i3 : float, i4 : char) returns (f : float);
let
    a = x * x;
    f = 2;
tel;

node f(i1 : int, i2 : int, i3 : float, i4 : char) returns (f : float);
let
    a = x * x;
    f = 2;
tel;
node otherfunction(i: int, a : int) returns (f: float);
let b   = x + x;
c = d;
tel;

node while(i : int) returns (g : int, t : int);
let
    a = x * x;
    f = 2 + 3;
    t = x + x * y + 2;
tel;
",
        );
    }
    #[test]
    fn t8() {
        ok_parse(
            "
node f(i1 : int, i2 : int, i3 : float, i4 : char) returns (f : float);
let
    a = x * x;
    f = 2;
tel;

node f(i1 : int, i2 : int, i3 : float, i4 : char) returns (f : float);
let
    a = x * x;
    f = 2;
tel;
node fu(i1 : int, i2 : int) returns (f: float);
let
    a = x * x;
    f = 2;
tel;
",
        );
    }

    #[test]
    fn t9() {
        ok_parse(
            "
node otherfunction(i: int, a : int) returns (f: float);
let b   = x + x;
c = d;
tel;

node while(i : int) returns (g : int, t : int);
let
    a = x * x;
    f = 2 + 3;
    t = x + x * y + 2;
tel;
",
        )
    }

    #[test]
    fn t10() {
        error_parse(
            "
node otherfunction(i: int, a : int) returns (f: float);
let b   = x + x;
c = ;
tel;
",
        )
    }

    #[test]
    fn t16() {
        error_parse(
            "
node ction( returns (f: float);
let
tel;
",
        )
    }
}
