use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

mod command;
mod store;

const SERVER_HOST: &str = "127.0.0.1";
const SERVER_PORT: u64 = 6379;

fn main() -> io::Result<()> {

    let address = format!("{}:{}", SERVER_HOST, SERVER_PORT);

    let listener = TcpListener::bind(&address)?;
    println!("Server listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_client(stream) {
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

fn handle_client(stream: TcpStream) -> io::Result<()> {
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

        writer.write_all(line.as_bytes())?;
    }

    println!("Client disconnected: {}", peer);
    Ok(())
}
