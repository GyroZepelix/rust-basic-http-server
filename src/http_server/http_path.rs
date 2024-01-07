use std::fmt::{Display, Formatter, write};
use std::ops::Deref;


#[derive(Debug)]
pub struct HttpPath {
    path: String,
    pub path_segments: Vec<HttpPathSegment>
}

#[derive(Debug)]
pub struct HttpPathSegment(pub String);

impl From<&str> for HttpPath {
    fn from(value: &str) -> Self {
        let path_segments = value.split('/')
            .filter(|path_segment| !path_segment.is_empty())
            .map(|path_segment| HttpPathSegment(path_segment.to_owned()))
            .collect();

        HttpPath {
            path: value.to_string(),
            path_segments
        }
    }
}

impl From<String>for HttpPath {
    fn from(value: String) -> Self {
        let path_segments = value.split('/')
            .filter(|path_segment| !path_segment.is_empty())
            .map(|path_segment| HttpPathSegment(path_segment.to_owned()))
            .collect();

        HttpPath {
            path: value,
            path_segments
        }
    }
}

impl HttpPath {
    pub fn as_str(&self) -> &str {
        self.path.as_str()
    }
}

pub trait ToPathString {
    fn to_path_string(&self) -> String;
}

impl Display for HttpPathSegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ToPathString for Vec<HttpPathSegment> {
    fn to_path_string(&self) -> String {
        self.iter().map(|segment| segment.0.deref()).collect::<Vec<_>>().join("/")
    }
}

impl ToPathString for [HttpPathSegment] {
    fn to_path_string(&self) -> String {
        self.iter().map(|segment| segment.0.deref()).collect::<Vec<_>>().join("/")
    }
}

impl From<&str> for HttpPathSegment {
    fn from(value: &str) -> Self {
        HttpPathSegment(value.to_owned())
    }
}

