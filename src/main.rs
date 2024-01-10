mod http_server;

use std::io::{Read, Write};
use std::net::TcpListener;
use std::ops::Deref;
use crate::http_server::http_path::ToPathString;
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

    let path_segments = &request.request_line.path.path_segments;
    let headers = &request.headers;

    let echo_response_builder = HttpResponse::builder()
        .add_header(("Content-Type", "text/plain"))
        .status_code(HttpStatusCode::Ok);

    let user_agent_response_builder = HttpResponse::builder()
        .add_header(("Content-Type", "text/plain"))
        .status_code(HttpStatusCode::Ok);


    match path_segments.get(0) {
        None => HttpStatusCode::Ok.into(),
        Some(path_segment) => match path_segment.0.deref() {
            "secret" => HttpStatusCode::Forbidden.into(),
            "echo" => echo_response_builder.body(&path_segments[1..].to_path_string()).build(),
            "user-agent" => user_agent_response_builder.body(headers.get("User-Agent").unwrap_or("")).build(),
            _ => HttpStatusCode::NotFound.into()
        }
    }

    // match request.request_line.path.as_str() {
    //     "/" => HttpStatusCode::Ok.into(),
    //     "/secret" => HttpStatusCode::Forbidden.into(),
    //     _ => HttpStatusCode::NotFound.into()
    // }
}
