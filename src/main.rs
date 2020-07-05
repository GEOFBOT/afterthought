#[macro_use]
extern crate lalrpop_util;

#[macro_use]
extern crate nom;

pub mod ast;
pub mod lexer;

use ast::{Program, Token};
lalrpop_mod!(pub postscript);

#[derive(PartialEq)]
enum StackItem {
    Int(i32),
    Real(f64),
    LiteralName(String),
    StringLiteral(String),
}

fn main() {}

fn execute(p: Program) -> Option<StackItem> {
    let mut cur = &p;
    let mut operand_stack = Vec::<StackItem>::new();
    loop {
        match cur {
            Program::End => {
                break;
            }
            Program::Concat(t, b) => {
                match t {
                    Token::Int(i) => {
                        operand_stack.push(StackItem::Int(*i));
                    }
                    Token::StringLiteral(s) => {
                        operand_stack.push(StackItem::StringLiteral(s.clone()));
                    }
                    Token::Name(n) => match n.as_str() {
                        "add" => {
                            let a = operand_stack.pop().unwrap();
                            let b = operand_stack.pop().unwrap();
                            match (a, b) {
                                (StackItem::Int(a), StackItem::Int(b)) => {
                                    operand_stack.push(StackItem::Int(a + b));
                                }
                                _ => {
                                    panic!("type error");
                                }
                            }
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

    assert!(postscript::StringLiteralTokenParser::new()
        .parse("(test)")
        .is_ok());
    assert!(postscript::StringLiteralTokenParser::new()
        .parse("(special characters *!&}^%)")
        .is_ok());
}

#[test]
fn test_many_tokens() {
    assert!(postscript::ProgramParser::new().parse("10 13 add").is_ok());
}

#[test]
fn test_add() {
    let p = postscript::ProgramParser::new().parse("10 11 add").unwrap();
    assert!(execute(p) == Some(StackItem::Int(10 + 11)));

    let p = postscript::ProgramParser::new()
        .parse("10 11 12 13 add add add")
        .unwrap();
    assert!(execute(p) == Some(StackItem::Int(10 + 11 + 12 + 13)));
}
