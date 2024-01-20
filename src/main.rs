mod http_server;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::ops::Deref;
use crate::http_server::http_path::ToPathString;
use crate::http_server::http_request::HttpMethod::GET;
use crate::http_server::http_request::HttpRequest;
use crate::http_server::http_response::{HttpResponse, HttpStatusCode};
use crate::http_server::lib::{HttpServer, RouteHandle};

fn main() {
    env_logger::init();

    let http_server = HttpServer::builder()
        .listener("127.0.0.1:4221")
        .add_route(RouteHandle::new(GET, "/secret", |req, var| HttpStatusCode::Forbidden.into()))
        .add_route(RouteHandle::new(GET, "/echo/{to_echo}", echo))
        .add_route(RouteHandle::new(GET, "/user-agent", user_agent))
        .build();

    http_server
        .run()
        .join()
        .unwrap();

}

fn echo(request: &HttpRequest, path_variables: &HashMap<String, String>) -> HttpResponse {
    let to_echo = path_variables.get("to_echo")
        .map_or("".to_string(), |var| var.to_string());

    HttpResponse::builder()
        .add_header(("Content-Type", "text/plain"))
        .status_code(HttpStatusCode::Ok)
        .body(&to_echo)
        .build()
}

fn user_agent(request: &HttpRequest, path_variables: &HashMap<String, String>) -> HttpResponse {
    let user_agent_header = request.headers.get("User-Agent").unwrap_or("");

    HttpResponse::builder()
        .add_header(("Content-Type", "text/plain"))
        .status_code(HttpStatusCode::Ok)
        .body(user_agent_header)
        .build()
}