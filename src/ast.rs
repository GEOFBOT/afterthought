pub enum Token {
    Int(i32),
    LiteralName(String),
    ImmName(String),
    Name(String),
}

pub enum Program {
    Concat(Token, Box<Program>),
    End,
}
