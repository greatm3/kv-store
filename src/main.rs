use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

mod command;
mod store;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;
    println!("Server listening on 127.0.0.1:6379");

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

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
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
