mod command;
mod store;

fn main() {
    let res = command::lexer("SET \"key spaced Great Ezenna");

    match res {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => println!("{e}"),
    }
}
