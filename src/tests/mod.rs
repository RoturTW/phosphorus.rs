use std::fmt::Write;
mod parser;
mod ast;
mod error;

use std::fs::{read_dir, read_to_string, DirEntry};
use std::io;
use colored::Colorize;
use crate::rtr::ast::node::EventTarget;
use crate::rtr::RTRInstance;
use crate::tests::ast::tokenise;
use crate::tests::parser::{Code, CodePart, Parser, Test};

pub enum TestResultOutput {
    Pass,
    Fail { expected: Vec<String>, got: String }
}

pub fn process_code(code: Vec<CodePart>, vars: &[String]) -> String {
    let mut out = String::new();
    
    for part in code {
        out = format!("{out}{}", match part {
            CodePart::Str(str) => str,
            CodePart::Var(idx) => vars.get(idx-1).unwrap_or(&String::from("null")).clone()
        });
    }
    
    out
}

pub fn run_test_file(file: &DirEntry) -> bool {
    let file_path = file.path();
    let file_name = file.file_name();
    let file_data = read_to_string(file_path)
        .expect("cannot read test file");
    
    let mut parser = Parser {
        pointer: 0,
        tokens: tokenise(&file_data)
    };
    
    let test = parser.parse();
    
    if let Err(err) = test {
        println!("{}", format!("err in {}: {err}", file_name.display()).bright_red());
        false
    } else if let Ok(test) = test {
        run_test(&test)
    } else { false }
}

pub fn run_test(test: &Test) -> bool {
    let test_name = &test.name;
    
    let total = test.result.len();
    let mut passed: usize = 0;
    
    let mut out_text = String::new();
    
    for (idx, result) in test.result.iter().enumerate() {
        let out = run_test_result(test, result.0.clone(), &result.1);
        
        if let Err(err) = out {
            if total > 1 {
                writeln!(&mut out_text, "{}", format!("  test {:<25} [x]", format!("{test_name}-{idx}")).bright_red()).unwrap();
                writeln!(&mut out_text, "{}", format!("      error: {err}").bright_red()).unwrap();
            } else {
                writeln!(&mut out_text, "{}", format!("    error: {err}").bright_red()).unwrap();
            }
        } else if let Ok(data) = out {
            if let TestResultOutput::Fail { got, expected } = data {
                if total > 1 {
                    writeln!(&mut out_text, "{}", format!("  test {:<26} [x]", format!("{test_name}-{idx}")).bright_red()).unwrap();
                    writeln!(&mut out_text, "{}", format!("    got:      {got}\n    expected: {}", expected.join("\n          or: ")).bright_red()).unwrap();
                } else {
                    writeln!(&mut out_text, "{}", format!("  got:      {got}\n  expected: {}", expected.join("\n        or: ")).bright_red()).unwrap();
                }
            } else {
                passed += 1;
                if total > 1 {
                    writeln!(&mut out_text, "{}", format!("  test {:<26} [✓]", format!("{test_name}-{idx}")).bright_green()).unwrap();
                }
            }
        }
    }
    
    if passed == total {
        println!("{}", format!("test {:<25} [✓]", test.name).bright_green());
    } else {
        println!("{}", format!("test {:<25} [x]", test.name).bright_red());
        print!("{out_text}");
    }
    
    passed == total
}

pub fn run_test_result(test: &Test, expected: Vec<String>, vars: &[String]) -> Result<TestResultOutput, String> {
    let mut inst = RTRInstance::new();
    
    let code = match test.code.clone() {
        Code::Expr(code) => format!("event(onload){{log(\n{}\n);}}", process_code(code, vars)),
        Code::Program(code) => process_code(code, vars)
    };
    
    let parse_out = inst.parse(&code);
    if let Err(err) = parse_out {
        return Err(err.to_string());
    }
    
    let out = inst.run_event_target(&EventTarget::Global {
        name: String::from("onload")
    });
    
    if let Err(err) = out {
        return Err(err.to_string());
    }
    
    let mut str = inst.logs
        .iter()
        .map(crate::rtr::log::RTRLog::format)
        .collect::<Vec<_>>()
        .join(" ");
    
    Ok(if expected.contains(&str) {
        TestResultOutput::Pass
    } else {
        TestResultOutput::Fail {
            expected,
            got: str
        }
    })
}

pub fn run_tests() {
    
    let mut total: usize = 0;
    let mut passed: usize = 0;
    
    let mut tests_dir = read_dir("./assets/tests")
        .expect("cannot read ./assets/tests/")
        .collect::<Vec<_>>();
    
    let mut queue: Vec<io::Result<DirEntry>> = Vec::new();
    queue.append(&mut tests_dir);
    
    while !queue.is_empty() {
        let entry = queue.pop().unwrap().unwrap();
        
        if entry.file_type().unwrap().is_dir() {
            let mut category_dir = read_dir(entry.path())
                .expect("cannot read dir")
                .collect::<Vec<_>>();
            
            queue.append(&mut category_dir);
        } else {
            let pass = run_test_file(&entry);
            if pass {
                passed += 1;
            }
            total += 1;
        }
    }
    
    println!("{}", format!("\n{passed}/{total} tests passed :P").bright_cyan());
}
