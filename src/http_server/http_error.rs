#[derive(Debug)]
pub enum HttpServerError {
    InvalidHttpRequestStructure,
    InvalidRequestLineSyntax,
    HttpMethodNotFound,
    InvalidHttpVersionFormat,
    HttpServerAlreadyRunning
}

pub type Result<T> = std::result::Result<T, HttpServerError>;
