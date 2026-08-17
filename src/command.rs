use std::error::Error;
use std::fmt;
use std::mem;

#[derive(Debug, PartialEq)]
pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Del { key: String },
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyCommand,
    UnknownCommand(String),
    WrongArgumentCount { expected: usize, got: usize },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyCommand => write!(f, "ERR empty command: no input provided\n"),
            ParseError::UnknownCommand(s) => {
                write!(f, "ERR unknown command '{s}': expected SET, GET or DEL\n")
            }
            ParseError::WrongArgumentCount { expected, got } => write!(
                f,
                "wrong number of arguments: expected {expected} arguments, got {got}\n"
            ),
        }
    }
}

// will eventually make a smarter lexer, this should do for now - todo
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Word(String),
    QuotedString(String),
}

impl Token {
    pub fn word(s: &str) -> Self {
        Token::Word(s.to_string())
    }

    pub fn quoted(s: &str) -> Self {
        Token::QuotedString(s.to_string())
    }

    // this is to extract the string
    pub fn into_inner(self) -> String {
        match self {
            Token::Word(s) | Token::QuotedString(s) => s,
        }
    }
}

// this lexer currently has one way to fail: reaching end of line while still inside a quote
#[derive(Debug, PartialEq)]
pub enum LexError {
    StillInQuote,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ERR Reached EOL while still inside a quote\n")
    }
}

impl Error for LexError {}
impl Error for ParseError {}

pub fn lexer(line: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::<Token>::new();
    let mut chars = line.chars().peekable();
    let mut current_word = String::new();

    let mut in_quotes = false;

    while let Some(&ch) = chars.peek() {
        if in_quotes {
            match ch {
                '"' => {
                    in_quotes = false;
                    chars.next();

                    // using mem::take to clone and clear the current_word at the same time.
                    tokens.push(Token::QuotedString(mem::take(&mut current_word)));
                }
                _ => {
                    current_word.push(ch);
                    chars.next();
                }
            }
        } else {
            match ch {
                ' ' => {
                    if current_word.len() > 0 {
                        tokens.push(Token::Word(mem::take(&mut current_word)));
                    }
                }
                '"' => {
                    in_quotes = true;
                    if current_word.len() > 0 {
                        tokens.push(Token::Word(mem::take(&mut current_word)));
                    }
                    chars.next();
                    continue;
                }
                _ => current_word.push(ch),
            }

            chars.next();
        }
    }

    if !in_quotes {
        if current_word.len() > 0 {
            tokens.push(Token::Word(mem::take(&mut current_word)));
        }
    } else {
        return Err(LexError::StillInQuote);
    }

    Ok(tokens)
}

pub fn parse(tokens: &[Token]) -> Result<Command, ParseError> {
    let supported_commands = vec!["SET".to_string(), "GET".to_string(), "DEL".to_string()];

    if tokens.is_empty() {
        return Err(ParseError::EmptyCommand);
    }

    match &tokens[0] {
        Token::QuotedString(s) => return Err(ParseError::UnknownCommand(s.to_string())),
        Token::Word(s) => {
            if !supported_commands.contains(&s.clone()) {
                return Err(ParseError::UnknownCommand(s.to_string()));
            }

            let _command = tokens[0].clone().into_inner();

            match _command.as_str() {
                "SET" => {
                    if tokens.len() != 3 {
                        return Err(ParseError::WrongArgumentCount {
                            expected: 2,
                            got: tokens.len() - 1,
                        });
                    }

                    let _key = tokens[1].clone().into_inner();
                    let _value = tokens[2].clone().into_inner();

                    return Ok(Command::Set {
                        key: _key,
                        value: _value,
                    });
                }
                "GET" => {
                    if tokens.len() != 2 {
                        return Err(ParseError::WrongArgumentCount {
                            expected: 1,
                            got: tokens.len() - 1,
                        });
                    }

                    let _key = tokens[1].clone().into_inner();

                    return Ok(Command::Get { key: _key });
                }
                "DEL" => {
                    if tokens.len() != 2 {
                        return Err(ParseError::WrongArgumentCount {
                            expected: 1,
                            got: tokens.len() - 1,
                        });
                    }

                    let _key = tokens[1].clone().into_inner();

                    return Ok(Command::Del { key: _key });
                }
                _ => {}
            }
        }
    }

    return Ok(Command::Set {
        key: "j".to_string(),
        value: "h".to_string(),
    });
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn test_simple_words() {
        let input = "SET name Great";
        let expected = vec![
            Token::Word("SET".to_string()),
            Token::Word("name".to_string()),
            Token::Word("Great".to_string()),
        ];

        assert_eq!(lexer(input).unwrap(), expected)
    }

    #[test]
    fn test_quotes_in_middle() {
        let input = "SET \"key spaced\" value";
        let expected = vec![
            Token::Word("SET".to_string()),
            Token::QuotedString("key spaced".to_string()),
            Token::Word("value".to_string()),
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
        let input = "SET     age       192   ";
        let expected = vec![
            Token::Word("SET".to_string()),
            Token::Word("age".to_string()),
            Token::Word("192".to_string()),
        ];

        assert_eq!(lexer(input).unwrap(), expected)
    }

    #[test]
    fn test_unterminated_quote_error() {
        let input = "GET \"name";
        let result = lexer(input);

        assert!(matches!(result, Err(LexError::StillInQuote)))
    }

    #[test]
    fn test_adjacent_quotes() {
        let input = "SET key\"value spaced\"";
        let expected = vec![
            Token::Word("SET".to_string()),
            Token::Word("key".to_string()),
            Token::QuotedString("value spaced".to_string()),
        ];

        assert_eq!(lexer(input).unwrap(), expected)
    }
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn test_parse_empty_tokens() {
        let tokens = vec![];
        assert_eq!(parse(&tokens), Err(ParseError::EmptyCommand))
    }

    #[test]
    fn test_parse_set_valid() {
        let tokens = vec![
            Token::word("SET"),
            Token::word("name"),
            Token::word("Great"),
        ];
        let expected = Command::Set {
            key: "name".to_string(),
            value: "Great".to_string(),
        };
        assert_eq!(parse(&tokens).unwrap(), expected)
    }

    #[test]
    fn test_parse_set_with_quoted_value() {
        let tokens = vec![
            Token::word("SET"),
            Token::word("bio"),
            Token::quoted("programmer"),
        ];
        let expected = Command::Set {
            key: "bio".to_string(),
            value: "programmer".to_string(),
        };
        assert_eq!(parse(&tokens).unwrap(), expected);
    }

    #[test]
    fn test_parse_get_valid() {
        let tokens = vec![Token::word("GET"), Token::word("name")];
        let expected = Command::Get {
            key: "name".to_string(),
        };
        assert_eq!(parse(&tokens).unwrap(), expected);
    }

    #[test]
    fn test_parse_del_valid() {
        let tokens = vec![Token::word("DEL"), Token::word("name")];
        let expected = Command::Del {
            key: "name".to_string(),
        };
        assert_eq!(parse(&tokens).unwrap(), expected);
    }

    #[test]
    fn test_parse_unknown_command() {
        let tokens = vec![Token::word("PING")];
        assert_eq!(
            parse(&tokens),
            Err(ParseError::UnknownCommand("PING".to_string()))
        );
    }

    #[test]
    fn test_parse_quoted_command_is_unknown() {
        // "SET" should not be treated as a valid command keyword, because it has quotes... i'll consider if to change later
        let tokens = vec![Token::quoted("SET"), Token::word("key"), Token::word("val")];
        assert_eq!(
            parse(&tokens),
            Err(ParseError::UnknownCommand("SET".to_string()))
        );
    }

    #[test]
    fn test_parse_wrong_args_set() {
        let tokens = vec![Token::word("SET"), Token::word("key")];
        assert_eq!(
            parse(&tokens),
            Err(ParseError::WrongArgumentCount {
                expected: 2,
                got: 1
            })
        );
    }

    #[test]
    fn test_parse_wrong_args_get() {
        let tokens = vec![Token::word("GET"), Token::word("key"), Token::word("extra")];
        assert_eq!(
            parse(&tokens),
            Err(ParseError::WrongArgumentCount {
                expected: 1,
                got: 2
            })
        );
    }
}
