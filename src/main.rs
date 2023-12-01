mod http_server;

use std::io::{Read, Write};
use std::net::TcpListener;
use crate::http_server::http_request::HttpRequest;
use crate::http_server::http_response::{HttpResponse, HttpStatusCode};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut buffer = [0; 128];
                stream.read(&mut buffer).unwrap();
                println!("response in bytes: {:?}", buffer);
                println!("response: {:?}", String::from_utf8_lossy(&buffer));
                let http_request = HttpRequest::from_bytes(&buffer).expect("Incoming message should have correct syntax");
                dbg!(&http_request);

                let http_response = handle_requests(&http_request);

                stream.write(http_response.to_bytes().as_slice()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_requests(request: &HttpRequest) -> HttpResponse {
    match request.request_line.path.as_str() {
        "/" => HttpStatusCode::Ok.into(),
        _ => HttpStatusCode::NotFound.into()
    }
}
