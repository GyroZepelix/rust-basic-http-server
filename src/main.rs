mod http_server;

use std::io::{Read, Write};
use std::net::TcpListener;
use crate::http_server::http_request::HttpRequest;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let buffer = [0; 128];
                println!("response in bytes: {:?}", buffer);
                println!("response: {:?}", String::from_utf8_lossy(&buffer));
                let http_request = HttpRequest::from_bytes(&buffer).expect("Incoming message should have correct syntax");
                dbg!(http_request);

                stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
