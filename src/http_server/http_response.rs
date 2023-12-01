use crate::create_enum_and_matchers;
use crate::http_server::http_version::HttpVersion;


#[derive(Default)]
pub struct HttpResponse {
    pub status_code: HttpStatusCode,
    pub http_version: HttpVersion
}

#[derive(Default)]
pub struct HttpResponseBuilder {
    status_code: HttpStatusCode,
    http_version: HttpVersion
}

impl HttpResponseBuilder {

    pub fn status_code(mut self, status_code: HttpStatusCode) -> Self {
        self.status_code = status_code;
        return self
    }

    pub fn http_version(mut self, http_version: HttpVersion) -> Self {
        self.http_version = http_version;
        return self
    }

    pub fn headers(mut self, headers: Vec<String>) -> Self {
        todo!()
    }

    pub fn add_header(mut self, header: String) -> Self {
        todo!()
    }

    pub fn build(self) -> HttpResponse{
        HttpResponse{
            status_code: self.status_code,
            http_version: self.http_version
        }
    }
}

impl HttpResponse {
    pub fn new(status_code: HttpStatusCode, http_version: HttpVersion) -> Self {
        HttpResponse {
            status_code,
            http_version,
        }
    }
    
    pub fn builder() -> HttpResponseBuilder {
        HttpResponseBuilder::default()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        format!("{} {} {} \r\n\r\n", self.http_version, self.status_code.to_int(), self.status_code.to_string()).as_bytes().to_vec()
    }
}

impl From<HttpStatusCode> for HttpResponse {
    fn from(value: HttpStatusCode) -> Self {
        Self {
            status_code: value,
            ..Self::default()
        }
    }
}

create_enum_and_matchers!(
    HttpStatusCode,
    Ok, 200, "OK",
    Created, 201, "Created",
    Accepted, 202, "Accepted",
    NoContent, 204, "No Content",
    MovedPermanently, 301, "Moved Permanently",
    Found, 302, "Found",
    SeeOther, 303, "See Other",
    NotModified, 304, "Not Modified",
    TemporaryRedirect, 307, "Temporary Redirect",
    PermanentRedirect, 308, "Permanent Redirect",
    BadRequest, 400, "Bad Request",
    Unauthorized, 401, "Unauthorized",
    Forbidden, 403, "Forbidden",
    NotFound, 404, "Not Found",
    MethodNotAllowed, 405, "Method Not Allowed",
    RequestTimeout, 408, "Request Timeout",
    Conflict, 409, "Conflict",
    Gone, 410, "Gone",
    PayloadTooLarge, 413, "Payload Too Large",
    UriTooLong, 414, "URI Too Long",
    UnsupportedMediaType, 415, "Unsupported Media Type",
    RangeNotSatisfiable, 416, "Range Not Satisfiable",
    InternalServerError, 500, "Internal Server Error",
    NotImplemented, 501, "Not Implemented",
    BadGateway, 502, "Bad Gateway",
    ServiceUnavailable, 503, "Service Unavailable",
    GatewayTimeout, 504, "Gateway Timeout",
    HttpVersionNotSupported, 505, "HTTP Version Not Supported"
);


