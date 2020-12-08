use super::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0 as char,
        };

        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0 as char;
        } else {
            if let Some(ch) = self.input.chars().nth(self.read_position) {
                self.ch = ch;
            }
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let token = match self.ch {
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '\u{0000}' | _ => Token::EOF,
        };

        self.read_char();
        token
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::Token;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let test = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
            Token::EOF,
        ];

        let mut lexer = Lexer::new(input);

        for (i, t) in test.iter().enumerate() {
            let tok = lexer.next_token();

            assert_eq!(
                tok, *t,
                "test[{}] - token is wrong. expected {:?} but got {:?}",
                i, t, tok
            );
        }
    }
}
