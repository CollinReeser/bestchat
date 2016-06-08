use std::io::BufRead;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::thread;

fn client_connection(stream: TcpStream, addr: SocketAddr) {
    let mut reader = BufReader::new(stream);

    let mut buf = String::new();

    loop {
        match reader.read_line(&mut buf) {
            Err (msg) => {
                println!("{}: EOF: {}", addr, msg);
                break;
            }
            Ok (0) => break,
            Ok (_) => {
                print!("{}: {}", addr, buf);
            },
        }

        buf.clear();
    }

    println!("Disconnected: {}", addr);
}

fn main () {
    let listener = TcpListener::bind("192.168.1.40:24567").unwrap();

    loop {
        match listener.accept() {
            Err(err) => println!("accept() Err ({})", err),
            Ok((stream, addr)) => {
                println!("Connected: {}", addr);

                thread::spawn(move || {
                    client_connection(stream, addr)
                });
            },
        }
    }
}
