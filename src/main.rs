use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

mod command;
mod store;

const SERVER_HOST: &str = "127.0.0.1";
const SERVER_PORT: u64 = 6379;

fn main() -> io::Result<()> {

    let mut kv_store = store::Store::new();

    let address = format!("{}:{}", SERVER_HOST, SERVER_PORT);

    let listener = TcpListener::bind(&address)?;
    println!("Server listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_client(stream, &mut kv_store) {
                    eprintln!("Error handling client: {}", e);
                }
            },

            Err(e) => {
                eprintln!("Connection failed: {}", e)
            }
        }
    }

    Ok(())
}

fn handle_client(stream: TcpStream, kv_store: &mut store::Store) -> io::Result<()> {
    let peer = stream.peer_addr()?;
    println!("New client connected: {}", peer);

    let reader_stream = stream.try_clone()?;
    let mut reader = BufReader::new(reader_stream);
    let mut writer = stream;

    let mut line = String::new();
    loop {
        line.clear();

        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        
        let tokens = match command::lexer(line.trim_end()) {
            Ok(tokens) => tokens,
            Err(lex_error) => {
                let error_response = lex_error.to_string();
                writer.write_all(error_response.as_bytes())?;
                writer.flush()?;
                continue;
            }
        };

        let parsed_command = match command::parse(&tokens) {
            Ok(cmd) => cmd,
            Err(parse_error) => {
                let error_response = parse_error.to_string();
                writer.write_all(error_response.as_bytes())?;
                writer.flush()?;
                continue;
            }
        };

        let response = format!("{}\n", kv_store.execute(parsed_command));
        writer.write_all(response.as_bytes())?;
        writer.flush()?;
    }

    println!("Client disconnected: {}", peer);
    Ok(())
}
