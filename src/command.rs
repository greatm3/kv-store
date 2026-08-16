// use std::string::ParseError;
use std::fmt;
use std::error::Error;

// will eventually make a smarter lexer, this should do for now - todo
#[derive(Debug)]
pub enum Token {
    Word(String),
    QuotedString(String)
}

// this lexer currently has one way to fail: reaching end of line while still inside a quote
#[derive(Debug)]
pub enum LexError {
    StillInQuote
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Reached EOL while still inside a quote")
    }
}

impl Error for LexError {}


pub fn lexer(line: &str) -> Result<Vec<Token>, LexError> {
    
}

// pub fn parse(tokens: &[Token]) -> Result<Command, ParseError> {

// }