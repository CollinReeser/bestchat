use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:6667").unwrap();

    loop {
        let mut line = String::new();

        print!("> ");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut line).unwrap();

        stream.write_all(&line.into_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
