use nom::{
    branch::alt,
    bytes::complete::{is_a, is_not, tag, take_while, take_while1, take_while_m_n},
    character::complete::one_of,
    character::is_digit,
    combinator::{complete, map_res, opt, peek},
    multi::{many1, separated_list},
    sequence::{preceded, tuple},
    IResult,
};
use std::str::FromStr;

use crate::ast::Token;

/// check if token ends with whitespace or eof
fn check_token_end(input: &str) -> IResult<&str, char> {
    if input.len() == 0 {
        Ok((input, '\n'))
    } else {
        // token can end in a comment
        one_of("\x00\x09\x0a\x0c\x0d\x20%")(input)
    }
}

fn token_int(input: &str) -> IResult<&str, Token> {
    let (input, sign) = opt(one_of("+-"))(input)?;
    let (input, num) = take_while1(|c: char| c.is_ascii_digit())(input)?;
    let (input, _) = peek(check_token_end)(input)?;
    let num = i32::from_str(num).unwrap();
    if sign == Some('-') {
        Ok((input, Token::Int(-1 * num)))
    } else {
        Ok((input, Token::Int(num)))
    }
}

fn token_name(input: &str) -> IResult<&str, Token> {
    let (input, s) = is_not("\x00\x09\x0a\x0c\x0d\x20()<>[]{}/%")(input)?;
    let (input, _) = peek(check_token_end)(input)?;
    Ok((input, Token::Name(s.to_owned())))
}

fn token_literal_name(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("/")(input)?;
    let (input, s) = is_not("\x00\x09\x0a\x0c\x0d\x20()<>[]{}/%")(input)?;
    let (input, _) = peek(check_token_end)(input)?;
    Ok((input, Token::LiteralName(s.to_owned())))
}
fn token_imm_name(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("//")(input)?;
    let (input, s) = is_not("\x00\x09\x0a\x0c\x0d\x20()<>[]{}/%")(input)?;
    let (input, _) = peek(check_token_end)(input)?;
    Ok((input, Token::ImmName(s.to_owned())))
}

fn token_comment_str(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("%")(input)?;
    let (input, s) = take_while(|c| c != '\n')(input)?;
    Ok((input, s))
}

fn postscript_token(input: &str) -> IResult<&str, Token> {
    alt((token_int, token_name, token_literal_name, token_imm_name))(input)
}

fn postscript_lex(input: &str) -> IResult<&str, Vec<Token>> {
    // TODO tokens can be separated by delimiters
    // i.e. (string)name is two tokens, a string literal and a name
    separated_list(
        many1(alt((is_a("\x00\x09\x0a\x0c\x0d\x20"), token_comment_str))),
        postscript_token,
    )(input)
    // still need to check if remaining string is all whitespace/comments!
}

#[test]
fn parse_int() {
    assert_eq!(token_int("100"), Ok(("", Token::Int(100))));
    assert_eq!(token_int("+100"), Ok(("", Token::Int(100))));
    assert_eq!(token_int("+100 "), Ok((" ", Token::Int(100))));
    assert!(token_int("+100a").is_err());
}

#[test]
fn parse_comment() {
    assert_eq!(token_comment_str("% abcdef"), Ok(("", " abcdef")));
    assert_eq!(token_comment_str("%\n"), Ok(("\n", "")));
}

#[test]
fn parse_token() {
    assert_eq!(postscript_token("100"), Ok(("", Token::Int(100))));
    assert_eq!(postscript_token("+100"), Ok(("", Token::Int(100))));
    assert_eq!(postscript_token("+100 "), Ok((" ", Token::Int(100))));
    assert_eq!(postscript_token("-100 "), Ok((" ", Token::Int(-100))));
    assert_eq!(postscript_token("-100 \n"), Ok((" \n", Token::Int(-100))));
    assert_eq!(postscript_token("-100%"), Ok(("%", Token::Int(-100))));
    assert_eq!(postscript_token("-100%\n"), Ok(("%\n", Token::Int(-100))));
    assert_eq!(
        postscript_token("+100a"),
        Ok(("", Token::Name("+100a".to_owned())))
    );
    assert_eq!(
        postscript_token("/+100a"),
        Ok(("", Token::LiteralName("+100a".to_owned())))
    );
    assert_eq!(
        postscript_token("//+100a"),
        Ok(("", Token::ImmName("+100a".to_owned())))
    );
}

#[test]
fn parse_many_tokens() {
    assert_eq!(postscript_lex("100"), Ok(("", vec![Token::Int(100)])));
    assert_eq!(
        postscript_lex("100 100a"),
        Ok(("", vec![Token::Int(100), Token::Name("100a".to_owned())]))
    );
    assert_eq!(
        postscript_lex("100 100a"),
        Ok(("", vec![Token::Int(100), Token::Name("100a".to_owned())]))
    );
    assert_eq!(
        postscript_lex("100 % comment\n 100a"),
        Ok(("", vec![Token::Int(100), Token::Name("100a".to_owned())]))
    );
    assert_eq!(
        postscript_lex("100% comment\n100a\n"),
        Ok(("\n", vec![Token::Int(100), Token::Name("100a".to_owned())]))
    );
}
