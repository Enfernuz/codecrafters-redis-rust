#![allow(unused_imports)]
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::thread;

use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        thread::spawn(|| match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => println!("error: {}", e),
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf: [u8; 256] = [0; 256];
    loop {
        let read_count: usize = stream.read(&mut buf).unwrap(); // blocking read
        if read_count == 0 {
            break;
        }
        stream.write(b"+PONG\r\n").unwrap();
    }
}
