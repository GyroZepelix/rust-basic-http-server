mod http_server;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::ops::Deref;
use crate::http_server::http_path::ToPathString;
use crate::http_server::http_request::HttpMethod::GET;
use crate::http_server::http_request::HttpRequest;
use crate::http_server::http_response::{HttpResponse, HttpStatusCode};
use crate::http_server::lib::{HttpServer, RequestContext, RouteHandle};

fn main() {
    //env_logger::init();

    let http_server = HttpServer::builder()
        .listener("127.0.0.1:4221")
        .add_route(RouteHandle::new(GET, "/", |cx| HttpStatusCode::Ok.into()))
        .add_route(RouteHandle::new(GET, "/secret", |cx| HttpStatusCode::Forbidden.into()))
        .add_route(RouteHandle::new(GET, "/echo/{to_echo}", echo))
        .add_route(RouteHandle::new(GET, "/echo/{to_echo}/{to_echo_two}", echo_two))
        .add_route(RouteHandle::new(GET, "/user-agent", user_agent))
        .build();

    http_server
        .run()
        .join()
        .unwrap();

}

fn echo(cx: &RequestContext) -> HttpResponse {
    let to_echo = cx.path_variables.get("to_echo")
        .map_or("".to_string(), |var| var.to_string());

    HttpResponse::builder()
        .add_header(("Content-Type", "text/plain"))
        .status_code(HttpStatusCode::Ok)
        .body(&to_echo)
        .build()
}

fn echo_two(cx: &RequestContext) -> HttpResponse {
    let to_echo = cx.path_variables.get("to_echo")
        .map_or("".to_string(), |var| var.to_string());
    let to_echo_two = cx.path_variables.get("to_echo_two")
        .map_or("".to_string(), |var| var.to_string());

    HttpResponse::builder()
        .add_header(("Content-Type", "text/plain"))
        .status_code(HttpStatusCode::Ok)
        .body(&format!("{}/{}", to_echo, to_echo_two))
        .build()
}

fn user_agent(cx: &RequestContext) -> HttpResponse {
    let user_agent_header = cx.http_request.headers.get("User-Agent").unwrap_or("");

    HttpResponse::builder()
        .add_header(("Content-Type", "text/plain"))
        .status_code(HttpStatusCode::Ok)
        .body(user_agent_header)
        .build()
}