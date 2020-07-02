#[macro_use]
extern crate lalrpop_util;

pub mod ast;

use ast::{Program, Token};
lalrpop_mod!(pub postscript);

fn main() {}

fn execute(p: Program) -> Option<i32> {
    let mut cur = &p;
    let mut operand_stack = Vec::<i32>::new();
    loop {
        match cur {
            Program::End => {
                break;
            }
            Program::Concat(t, b) => {
                match t {
                    Token::Int(i) => {
                        operand_stack.push(*i);
                    }
                    Token::Name(n) => match n.as_str() {
                        "add" => {
                            let a = operand_stack.pop().unwrap();
                            let b = operand_stack.pop().unwrap();
                            operand_stack.push(a + b);
                        }
                        _ => {
                            unimplemented!();
                        }
                    },
                    _ => {
                        unimplemented!();
                    }
                };

                cur = b;
            }
        }
    }

    return operand_stack.pop();
}

#[test]
fn test_token_terminals() {
    assert!(postscript::NameTokenParser::new().parse("aaaa").is_ok());
    assert!(postscript::NameTokenParser::new().parse("11aaaa").is_ok());
    assert!(postscript::NameTokenParser::new().parse("11").is_err());

    assert!(postscript::IntTokenParser::new().parse("11aaaa").is_err());
    assert!(postscript::IntTokenParser::new().parse("11").is_ok());

    assert!(postscript::ImmNameTokenParser::new()
        .parse("11aaaa")
        .is_err());
    assert!(postscript::ImmNameTokenParser::new()
        .parse("//11aaaa")
        .is_ok());

    assert!(postscript::LiteralNameTokenParser::new()
        .parse("/11aaaa")
        .is_ok());
    // blank literal name is valid
    assert!(postscript::LiteralNameTokenParser::new().parse("/").is_ok());
}

#[test]
fn test_many_tokens() {
    assert!(postscript::ProgramParser::new().parse("10 13 add").is_ok());
}

#[test]
fn test_basic() {
    let p = postscript::ProgramParser::new().parse("10 11 add").unwrap();
    assert!(execute(p) == Some(10 + 11));

    let p = postscript::ProgramParser::new()
        .parse("10 11 12 13 add add add")
        .unwrap();
    assert!(execute(p) == Some(10 + 11 + 12 + 13));
}
