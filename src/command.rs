use std::error::Error;
use std::fmt;
use std::string::ParseError;

// will eventually make a smarter lexer, this should do for now - todo
#[derive(Debug, PartialEq)]
pub enum Token {
    Word(String),
    QuotedString(String),
}

// this lexer currently has one way to fail: reaching end of line while still inside a quote
#[derive(Debug, PartialEq)]
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
    let mut current_word = String::new();

    let mut in_quotes = false;

    while let Some(&char) = chars.peek() {
        if in_quotes {
            match char {
                '"' => {
                    in_quotes = false;
                    chars.next();
                }
                _ => {
                    current_word.push(char);
                    chars.next();
                }
            }
        } else {
            match char {
                ' ' => {
                    tokens.push(Token::Word(current_word.clone()));
                    current_word.clear();
                }
                '"' => {
                    in_quotes = true;
                    chars.next();
                    continue;
                }
                _ => current_word.push(char),
            }

            chars.next();
        }
    }

    if !in_quotes {
        tokens.push(Token::Word(current_word.clone()));
    } else {
        return Err(LexError::StillInQuote);
    }

    Ok(tokens)
}

// pub fn parse(tokens: &[Token]) -> Result<Command, ParseError> {

// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_words() {
        let input = "SET key value";
        let expected = vec![
            Token::Word("SET".to_string()),
            Token::Word("key".to_string()),
            Token::Word("value".to_string())
        ];

        assert_eq!(lexer(input).unwrap(), expected)
    }

    #[test]
    fn test_quotes_in_middle() {
        let input = "SET \"key spaced\" value";
        let expected = vec![
            Token::Word("SET".to_string()),
            Token::Word("key spaced".to_string()),
            Token::Word("value".to_string())
        ];

        assert_eq!(lexer(input).unwrap(), expected)
    }

    #[test]
    fn test_trailing_quotes() {
        let input = "GET \"name\"";
        let expected = vec![
            Token::Word("GET".to_string()),
            Token::QuotedString("name".to_string()),
        ];

        assert_eq!(lexer(input).unwrap(), expected)
    }

    #[test]
    fn test_multiple_spaces() {
        let input = "SET     key       value   ";
        let expected = vec![
            Token::Word("SET".to_string()),
            Token::Word("key".to_string()),
            Token::Word("value".to_string())
        ];

        assert_eq!(lexer(input).unwrap(), expected)
    }

    #[test]
    fn test_unterminated_quote_error() {
        let input = "GET \"name";
        let result = lexer(input);

        assert!(matches!(result, Err(LexError::StillInQuote)))
    }
}