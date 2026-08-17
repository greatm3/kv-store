mod command;
mod store;

use command::{Token, parse};

fn main() {
    let sp = vec![
        Token::Word(String::from("set")),
        Token::Word(String::from("GET")),
    ];

    let result = parse(&sp);

    match result {
        Ok(s) => println!("{:?}", s),
        Err(e) => println!("{e}"),
    }
}
