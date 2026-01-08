use crate::{
    ast::double_visitor::{DoubleTogetherVisitor, ShallowEq},
    parser::{ast::ast, lustre_parser::lustre_parse, span::LSpan},
};
use colored::Colorize;
use serde_derive::{Deserialize, Serialize};
use std::io::Write;
use test_each_file::test_each_file;

test_each_file! { for ["lus", "json"] in "./tests" => test }

#[derive(Serialize, Deserialize)]
enum TestType {
    Pass,
    Fail,
}

#[derive(Deserialize, Serialize)]
struct TestInfo {
    parse: Option<TestType>,
    check: Option<TestType>,
    test: Option<TestType>,
}
fn test([lustre_file, json_info]: [&str; 2]) {
    use TestType::*;
    let test_info: TestInfo = serde_json::from_str(json_info).unwrap();

    match test_info.parse {
        Some(Pass) => {
            ok_parse(lustre_file);
            match test_info.check {
                Some(Pass) => {
                    ok_check(lustre_file);
                    match test_info.test {
                        Some(Pass) => {
                            ok_interpretation(lustre_file);
                        }
                        Some(Fail) => {
                            error_interpretation(lustre_file);
                        }
                        None => {}
                    }
                }
                Some(Fail) => {
                    error_check(lustre_file);
                }
                None => {}
            }
        }
        Some(Fail) => {
            error_parse(lustre_file);
        }
        None => {}
    }
}

pub fn ok_interpretation(input: &str) {
    ok_check(input);
    let build_ast = lustre_parse(input).unwrap();

    let (const_ast, _) = build_ast.propagate_const();
    println!("{}\n{}", ">> Propagate Constant :".blue(), const_ast);

    for node in const_ast.nodes.iter() {
        if node.is_test() && !node.is_only_true_equations() {
            panic!()
        }
    }
}

pub fn error_interpretation(input: &str) {
    ok_check(input);
    let build_ast = lustre_parse(input).unwrap();
    let (const_ast, _) = build_ast.propagate_const();
    println!("{}\n{}", ">> Propagate Constant :".blue(), const_ast);

    for node in const_ast.nodes.iter() {
        if node.is_test() && node.is_only_true_equations() {
            panic!()
        }
    }
}

/// Verify that the given lustre program :
/// - parse
/// - don't have any diagnostic from the type checker
pub fn ok_check(input: &str) {
    ok_parse(input);
    let mut build_ast = lustre_parse(input).unwrap();

    let (diags, _) = build_ast.check();
    if !diags.is_empty() {
        eprintln!(
            "{} : diagnostics should be empty\n>>{} = {:#?}",
            ">> ERROR".red(),
            "DIAGS".blue(),
            diags
        );
        panic!()
    }
}

pub fn error_check(input: &str) {
    ok_parse(input);
    let _ = std::io::stdout().flush();
    let mut build_ast = lustre_parse(input).unwrap();

    let (diags, _) = build_ast.check();
    if diags.is_empty() {
        println!(
            "{}: error expected but no diagnos + 1;tics",
            ">> ERROR".red()
        );
        panic!()
    }
}

pub fn error_parse(input: &str) {
    let span = LSpan::new(input);

    if let Ok((_, res)) = ast(span) {
        println!("{}\n{input}", ">> input : ".blue());
        println!("{}\n{res}", ">> result : ".red());
        panic!();
    }
}

pub fn ok_parse(input: &str) {
    struct Test {
        name: &'static str,
        passed: bool,
    }
    println!("\n{}\n\"{input}\"", ">> in :".blue());

    let mut tests = [
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

    match lustre_parse(input) {
        Err(err) => {
            println!("\n{} parsing of \"in\": {:#?}", "FAILED".red(), err);
        }
        Ok(in_parse) => {
            let in_parse_display = format!("{in_parse}");
            tests[0].passed = in_parse_display == input;

            println!("{}\n{in_parse}", ">> in | parse :".blue());

            // because we are on the ok case
            // and lustre_parse checks that the rest parse input is empty
            tests[1].passed = true;

            match lustre_parse(&in_parse_display) {
                Err(err) => {
                    println!(
                        "\n{} parsing of \"in | parse | display\": {:#?}",
                        "FAILED".red(),
                        err
                    );
                }
                Ok(in_parse_display_parse) => {
                    let mut shallow_eq = ShallowEq::default();
                    shallow_eq.walk(&in_parse, &in_parse_display_parse);
                    tests[2].passed = shallow_eq.is_eq();

                    // in_parse_display_parse == in_parse;
                    let in_parse_display_parse_display = format!("{in_parse_display_parse}");
                    tests[3].passed = in_parse_display_parse_display == in_parse_display;

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
        panic!()
    }
}
