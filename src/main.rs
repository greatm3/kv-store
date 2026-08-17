mod command;
mod store;

use command::Token;

fn main() {
    let sp = vec![
        Token::Word(String::from("SET")),
        Token::Word(String::from("GET")),
    ];

    let result = sp.contains(&Token::Word("SET".to_string()));

    println!("{}", result)
}
