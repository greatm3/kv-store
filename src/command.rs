use std::error::Error;
use std::fmt;
use std::string::ParseError;

// will eventually make a smarter lexer, this should do for now - todo
#[derive(Debug)]
pub enum Token {
    Word(String),
    QuotedString(String),
}

// this lexer currently has one way to fail: reaching end of line while still inside a quote
#[derive(Debug)]
pub enum LexError {
    StillInQuote,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Reached EOL while still inside a quote")
    }
}

impl Error for LexError {}

pub fn lexer(line: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::<Token>::new();
    let mut chars = line.chars().peekable();

    // let in_quotes = false;

    while let Some(&char) = chars.peek() {
        if char.is_whitespace() {
            chars.next();
        } else {
            let mut current_word = String::new();

            while let Some(&c_char) = chars.peek() {
                if c_char.is_alphanumeric() {
                    current_word.push(c_char);
                    chars.next();
                } else {
                    break;
                }
            }

            tokens.push(Token::Word(current_word));
        }
    }

    Ok(tokens)
}

// pub fn parse(tokens: &[Token]) -> Result<Command, ParseError> {

// }
