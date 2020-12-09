use super::lexer::Lexer;
use super::token::Token;
use std::io::{stdin, stdout, Write};

const PROMPT: &str = ">> ";

pub fn start() {
    loop {
        let mut s = String::new();
        print!("{}", PROMPT);
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");

        let mut lexer = Lexer::new(&s);
        let mut token = lexer.next_token();

        while token != Token::EOF {
            println!("{}", token);
            token = lexer.next_token();
        }
    }
}
