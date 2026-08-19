use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

mod command;
mod store;

fn main() {
    
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
