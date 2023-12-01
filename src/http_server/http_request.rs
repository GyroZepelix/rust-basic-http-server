#[derive(Debug)]
pub struct HttpRequest {
    pub request_line: RequestLine,
    pub headers: Option<Vec<HttpRequestHeader>>
}

#[derive(Debug)]
pub struct RequestLine {
    pub method: HttpMethod,
    pub path: String,
    pub http_version: String
}

#[derive(Debug)]
pub enum HttpServerError {
    InvalidHttpRequestStructure,
    InvalidRequestLineSyntax,
    HttpMethodNotFound
}

const NEWLINE: u8 = 10;
const EMPTY_LINE: u8 = 32;

impl HttpRequest {

    /// Takes in the bytes received from a TcpStream and trys to convert them into a HttpRequest
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, HttpServerError> {
        let split_lines = split_lines_by_char(bytes, NEWLINE);
        let request_line = RequestLine::from_bytes(split_lines.first().ok_or(HttpServerError::InvalidRequestLineSyntax)?)?;

        Ok(Self {
            request_line,
            headers: None
        })
    }
}

impl RequestLine {
    pub fn from_bytes(line_bytes: &[u8]) -> Result<Self, HttpServerError> {
        let request_line_bytes: Vec<&[u8]> = split_lines_by_char(line_bytes, EMPTY_LINE);

        if request_line_bytes.len() != 3 { return Err(HttpServerError::InvalidRequestLineSyntax) };

        let method = HttpMethod::from_bytes(request_line_bytes[0]).ok_or(HttpServerError::HttpMethodNotFound)?;
        let path = String::from_utf8_lossy(request_line_bytes[1]).to_string();
        let http_version = String::from_utf8_lossy(request_line_bytes[2]).to_string();

        Ok(Self {
            method,
            path,
            http_version
        })
    }
}

fn split_lines_by_char(bytes: &[u8], splitter: u8) -> Vec<&[u8]> {
    bytes
        .split(|byte| *byte == splitter)
        .collect()
}


#[derive(Debug)]
pub struct HttpRequestHeader(String, String);
macro_rules! create_enum_and_matchers {
    ($enum_name:ident, $($enum_options:ident),*) => {
        #[derive(Debug)]
        pub enum $enum_name {
            $($enum_options,)*
        }

        impl $enum_name {
            pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
                match bytes {
                    $(
                        bytes if bytes == stringify!($enum_options).as_bytes() => Some($enum_name::$enum_options),
                    )*
                    _ => None
                }
            }
        }
    };
}

create_enum_and_matchers!(HttpMethod, GET, POST, PUT, DELETE, OPTION, HEAD);

