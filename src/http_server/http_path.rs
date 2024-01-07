#[derive(Debug)]
pub struct HttpPath {
    path: String,
    pub path_segments: Vec<String>
}

impl From<&str> for HttpPath {
    fn from(value: &str) -> Self {
        let path_segments = value.split('/')
            .filter(|path_segment| !path_segment.is_empty())
            .map(|path_segment| path_segment.to_owned())
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
            .map(|path_segment| path_segment.to_owned())
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


