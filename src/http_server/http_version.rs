use std::fmt::Display;
use crate::http_server::helper::split_lines_by_char;
use crate::http_server::http_error::HttpServerError;
use crate::http_server::http_error::HttpServerError::InvalidHttpVersionFormat;

#[derive(Debug)]
pub struct HttpVersion {
    name: String,
    major: String,
    minor: String,
}

const BACKSLASH: u8 = 47;
const DOT: u8 = 46;

impl HttpVersion {

    pub fn new(name: String, major: String, minor: String) -> Self {
        HttpVersion{
            name,
            major,
            minor
        }
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, HttpServerError> {
        let name_and_version = split_lines_by_char(bytes, BACKSLASH);
        if name_and_version.len() != 2 { return Err(InvalidHttpVersionFormat) }

        let version_split = split_lines_by_char(name_and_version[1], DOT);
        if version_split.len() != 2 { return Err(InvalidHttpVersionFormat) }

        let name = String::from_utf8_lossy(name_and_version[0]).to_string();
        let major = String::from_utf8_lossy(version_split[0]).to_string();
        let minor = String::from_utf8_lossy(version_split[1]).to_string();

        Ok(HttpVersion {
            name,
            major,
            minor
        })
    }
}
impl Default for HttpVersion {
    fn default() -> Self {
        Self::new(
            "HTTP".to_string(),
            "1".to_string(),
            "1".to_string()
        )
    }
}
impl Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}/{}.{}", self.name, self.major, self.minor))
    }
}
