use std::collections::{HashMap, VecDeque};
use std::io::{Read, Seek, Write};
use std::net::{TcpListener, TcpStream};
use std::ops::{Add, Deref};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use log::{debug, error, info, trace};
use crate::http_server::http_request::{HttpMethod, HttpRequest};
use crate::http_server::http_response::HttpResponse;
use crate::http_server::http_error::Result;
use crate::http_server::http_path::{PathCompareResult, RouteMappingPath};
use crate::http_server::http_path::PathCompareResult::NotMatching;
use crate::http_server::http_response::HttpStatusCode::NotFound;

pub struct HttpServer {
    listener: Arc<TcpListener>,
    request_handler: Arc<RequestHandler>
}

#[derive(Default)]
pub struct HttpServerBuilder {
    listener_addr: Option<String>,
    route_handles: Vec<RouteHandle>
}

pub struct RouteHandle {
    pub method: HttpMethod,
    pub route: RouteMappingPath,
    pub function: Box<dyn Fn(&RequestContext) -> HttpResponse + Send + Sync>
}

pub struct RequestContext<'a> {
    pub http_request: &'a HttpRequest,
    pub path_variables: HashMap<String, String>
}
struct RequestHandler {
    route_handles: Vec<RouteHandle>
}

impl<'a> RequestContext<'a> {
    pub fn new(http_request: &'a HttpRequest, path_variables: HashMap<String, String>) -> RequestContext<'a> {
        Self {
            http_request,
            path_variables
        }
    }
}

impl RouteHandle {
    pub fn new<F: Fn(&RequestContext) -> HttpResponse + 'static + Send + Sync>(method: HttpMethod, route: &str, function: F) -> Self {
        RouteHandle {
            method,
            route: route.into(),
            function: Box::new(function)
        }
    }
}

impl HttpServerBuilder {
    pub fn listener<T: ToString>(mut self, addr: T) -> Self {
        self.listener_addr = Some(addr.to_string());
        self
    }

    pub fn add_route(mut self, route_handle: RouteHandle) -> Self {
        self.route_handles.push(route_handle);
        self
    }

    pub fn build(self) -> HttpServer {
        let addr = self.listener_addr.unwrap_or("".to_string());

        HttpServer {
            listener: Arc::new(
                TcpListener::bind(&addr)
                    .expect("TcpListener failed to bind to ip and")
            ),
            request_handler: Arc::new(self.route_handles.into())
        }

    }
    
}

impl HttpServer {
    pub fn builder() -> HttpServerBuilder {
        HttpServerBuilder::default()
    }

    pub fn run(self) -> JoinHandle<()> {

        info!("Server listening on: {}", self.listener.local_addr().expect("Listening address doesnt exist").to_string());
        let tcp_listener = self.listener.clone();
        let request_handler = self.request_handler.clone();

        let join_handle = thread::spawn(move || {
            for stream_result in tcp_listener.incoming() {
                match stream_result {
                    Ok(result) => {
                        let handler_clone = request_handler.clone();
                        thread::spawn(move || HttpServer::handle_tcp_stream(result, handler_clone));
                    },
                    Err(err) => {
                        error!("Invalid TcpStream received: {}", err);
                    }
                };
            }
        });
        join_handle
    }

    fn handle_tcp_stream(mut tcp_stream: TcpStream, request_handler: Arc<RequestHandler>) {
        debug!("Accepted new connection from: {}", tcp_stream.peer_addr().expect("Invalid peer address"));

        let mut buffer = [0; 128];
        tcp_stream.read(&mut buffer).unwrap();

        trace!("Response in bytes: {:?}", buffer);
        trace!("Response: {:?}", String::from_utf8_lossy(&buffer));
        let http_request = HttpServer::read_http_request(&buffer)
            .map_err(|err| {
                error!("Error while deserializing HttpRequest: {:?}", err)
            }).unwrap();
        debug!("{:#?}", http_request);

        let http_response = request_handler.handle(&http_request);
        HttpServer::send_http_response(tcp_stream, http_response);
    }

    fn send_http_response(mut tcp_stream: TcpStream, http_response: HttpResponse) {
        tcp_stream.write(http_response.to_bytes().as_slice()).unwrap();
    }

    fn read_http_request(buffer: &[u8]) -> Result<HttpRequest> {
        HttpRequest::from_bytes(&buffer)
    }
}

impl RequestHandler {
    fn handle(&self, http_request: &HttpRequest) -> HttpResponse {
        let path = &http_request.request_line.path;
        let routes_available: Vec<&RouteHandle> = self.route_handles.iter()
            .filter(|route_handle| route_handle.method == http_request.request_line.method)
            .collect();

        let found_route = routes_available.iter()
            .find_map(|route| {
                match route.route.matches(path) {
                    NotMatching => None,
                    PathCompareResult::Matching => Some((route, HashMap::default())),
                    PathCompareResult::MatchingWithVariables(variables_map) => Some((route, variables_map))
                }
            });


        // Run the endpoint function
        match found_route {
            Some((route, variables_map)) => (route.function)(&RequestContext::new(http_request, variables_map)),
            None => NotFound.into()
        }
    }
}

impl From<Vec<RouteHandle>> for RequestHandler {
    fn from(value: Vec<RouteHandle>) -> Self {
        RequestHandler {
            route_handles: value,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::http_server::lib::*;

    #[test]
    fn should_bind_server_to_ip() {
        let http_server = HttpServer::builder()
            .listener("127.0.0.1:65500")
            .build();

        assert_eq!(http_server.listener.local_addr().unwrap().to_string(), "127.0.0.1:65500");
    }


}