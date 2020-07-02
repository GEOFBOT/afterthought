#[derive(Debug)]
pub enum Token {
    Int(i32),
    LiteralName(String),
    ImmName(String),
    Name(String),
}

#[derive(Debug)]
pub enum Program {
    Concat(Token, Box<Program>),
    End,
}
