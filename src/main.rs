#[macro_use]
extern crate lalrpop_util;

pub mod ast;

lalrpop_mod!(pub postscript);

fn main() {}

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
