#[derive(Debug)]
pub enum HttpServerError {
    InvalidHttpRequestStructure,
    InvalidRequestLineSyntax,
    HttpMethodNotFound,
    InvalidHttpVersionFormat,
}