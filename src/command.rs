use std::string::ParseError;

// will eventually make a smarter lexer, this should do for now - todo
enum Token {
    Word(String),
    QuotedString(String)
}


pub fn lexer(line: &str) -> Result<Vec<Token>, LexError> {

}

pub fn parse(tokens: &[Token]) -> Result<Command, ParseError> {

}