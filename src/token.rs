#[derive(Eq, PartialEq, Debug)]
pub enum Token {
    Illegal(char),
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

pub fn lookup_keyword(ident: &str) -> Option<Token> {
    match ident {
        "fn" => Some(Token::Function),
        "let" => Some(Token::Let),
        _ => None,
    }
}
