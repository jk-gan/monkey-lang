#[derive(Eq, PartialEq, Debug)]
pub enum Token {
    Illegal,
    EOF,

    // Identifier + literals
    Ident(String),
    Int(i64),

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
}
