mod command;
mod store;

fn main() {
    let res = command::lexer("SET \"name\" Great");

    match res {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        },
        Err(e) => println!("{e}")
    }
}
