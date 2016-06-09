use std::io::BufRead;
use std::io::Read;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::thread;

extern crate bestchat;

use bestchat::message::*;

fn client_connection(stream: TcpStream, addr: SocketAddr) {
    let mut reader = BufReader::new(stream);

    let mut buf = Vec::new();

    loop {
        match reader.read_until(1u8, &mut buf) {
            Err (msg) => {
                println!("{}: EOF: {}", addr, msg);
                break;
            }
            Ok (0) => break,
            Ok (_) => {
                print!("{}: {}", addr, bytes_to_message(&buf[..buf.len()-1]));
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
