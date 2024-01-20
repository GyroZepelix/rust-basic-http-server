use std::collections::HashMap;
use crate::create_enum_and_matchers;
use crate::http_server::helper::{split_lines_by_byte, split_lines_by_bytes};
use crate::http_server::http_path::HttpPath;
use crate::http_server::http_version::HttpVersion;
use crate::http_server::HttpServerError;
use crate::http_server::http_error::Result;

#[derive(Debug)]
pub struct HttpRequestHeader(HashMap<String, String>);

#[derive(Debug)]
pub struct HttpRequest {
    pub request_line: RequestLine,
    pub headers: HttpRequestHeader
}

/// The first line of a HttpRequest
#[derive(Debug)]
pub struct RequestLine {
    pub method: HttpMethod,
    pub path: HttpPath,
    pub http_version: HttpVersion
}

const NEWLINE: u8 = b'\n';
const CARRIAGE_RETURN: u8 = b'\r';
const EMPTY_LINE: u8 = b' ';
const COLON: u8 = b':';

impl HttpRequest {

    /// Takes in the bytes received from a TcpStream and trys to convert them into a HttpRequest
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let split_lines = split_lines_by_bytes(bytes, &[CARRIAGE_RETURN, NEWLINE]);
        let http_lines = split_lines.len();
        let request_line = RequestLine::from_bytes(split_lines.first().ok_or(HttpServerError::InvalidRequestLineSyntax)?)?;
        let headers = HttpRequestHeader::from_bytes(&split_lines[1..http_lines-2]);

        Ok(Self {
            request_line,
            headers
        })
    }
}

impl HttpRequestHeader {

    /// Returns the value of a header if it exists
    pub fn get(&self, header_key: &str) -> Option<&str> {
        self.0.get(header_key)
            .map(|value| value.as_str())
    }
}

impl RequestLine {
    /// Takes in a slice of bytes and converts them into a RequestLine
    pub fn from_bytes(line_bytes: &[u8]) -> Result<Self> {
        let request_line_bytes: Vec<&[u8]> = split_lines_by_byte(line_bytes, EMPTY_LINE);

        if request_line_bytes.len() != 3 { return Err(HttpServerError::InvalidRequestLineSyntax) };

        let method = HttpMethod::from_bytes(request_line_bytes[0]).ok_or(HttpServerError::HttpMethodNotFound)?;
        let path = String::from_utf8_lossy(request_line_bytes[1]).to_string().into();
        let http_version = HttpVersion::from_bytes(request_line_bytes[2])?;

        Ok(Self {
            method,
            path,
            http_version
        })
    }
}

impl HttpRequestHeader {

    /// Takes in a slice of bytes and converts them into a HashMap of headers
    pub fn from_bytes(line_bytes: &[&[u8]]) -> Self {

        let lines: HashMap<String, String> = line_bytes.iter()
            .map(|bytes| String::from_utf8_lossy(bytes).to_string())
            .filter_map(|header_line| {
                header_line.split_once(':')
                    .map(|(key, value)| (key.to_string(), value.trim().to_string()))
            })
            .collect();

        Self(
            lines
        )
    }
}


create_enum_and_matchers!(HttpMethod, GET, POST, PUT, DELETE, OPTION, HEAD);


