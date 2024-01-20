use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use log::{error, warn};
use crate::http_server::http_path::PathCompareResult::{Matching, MatchingWithVariables, NotMatching};


#[derive(Debug, Clone)]
pub struct HttpPath {
    path: String,
    pub path_segments: Vec<HttpPathSegment>
}

#[derive(Debug, Clone)]
pub struct HttpPathSegment(pub String);


pub struct RouteMappingPath {
    path: String,
    pub path_segments: Vec<RouteMappingPathSegment>
}
#[derive(Debug)]
pub enum RouteMappingPathSegment{
    Constant(String),
    Variable(String)
}

#[cfg_attr(
    test,
    derive(Debug)
)]
#[derive(PartialEq)]
pub enum PathCompareResult {
    NotMatching,
    Matching,
    MatchingWithVariables(HashMap<String, String>)
}

impl RouteMappingPath {
    pub fn matches(&self, http_path: &HttpPath) -> PathCompareResult {
        type R = RouteMappingPathSegment;

        let route_segments = &self.path_segments;
        let path_segments = &http_path.path_segments;

        if route_segments.len() != path_segments.len() {
            return NotMatching
        };

        let mut variables = HashMap::new();

        for (i, segment) in route_segments.iter().enumerate() {
            let current_segment = &path_segments[i].0;
            match segment {
                RouteMappingPathSegment::Constant(constant_path) => {
                    if current_segment != constant_path {
                        return NotMatching
                    };
                }
                RouteMappingPathSegment::Variable(name) => {
                    variables.insert(name.clone(), current_segment.clone());
                }
            }
        }

        if variables.len() == 0 {
            Matching
        } else {
            MatchingWithVariables(variables)
        }
    }
}

impl From<&str> for RouteMappingPath {
    fn from(value: &str) -> Self {
        let mut variable_names = HashSet::new();

        let path_segments = value.split('/')
            .filter(|path_segment| !path_segment.is_empty())
            .map(|path_segment| {
                if !path_segment.starts_with('{') || !path_segment.ends_with('}') {
                    RouteMappingPathSegment::Constant(path_segment.to_string())
                } else  {
                    let name = trim_edges(path_segment).to_string();
                    if !variable_names.insert(name.clone()) {
                        error!("RouteMappingPath has duplicate path variable name: {}, \"{}\"", value, name);
                    }
                    if path_segment.len() == 2 {
                        warn!("RouteMappingPath has empty name for path variable: {}", value);
                    }
                    RouteMappingPathSegment::Variable(name)
                }
            })
            .collect();

        RouteMappingPath {
            path: value.to_string(),
            path_segments
        }
    }
}

impl From<&str> for HttpPath {
    fn from(value: &str) -> Self {
        let path_segments = value.split('/')
            .filter(|path_segment| !path_segment.is_empty())
            .map(|path_segment| {
                HttpPathSegment(path_segment.to_string())
            })
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

fn trim_edges(str: &str) -> &str {
    if str.len() <= 2 {
        ""
    } else {
        &str[1..str.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::http_server::http_path::*;

    #[test]
    fn should_match_basic_path() {
        assert_eq!(
            RouteMappingPath::from("/hello/world").matches(&"/hello/world".into()),
            Matching
        );
        assert_eq!(
            RouteMappingPath::from("/").matches(&"/".into()),
            Matching
        );
        assert_eq!(
            RouteMappingPath::from("/test").matches(&"/test".into()),
            Matching
        );
        assert_eq!(
            RouteMappingPath::from("/test/").matches(&"/test/".into()),
            Matching
        );
    }

    #[test]
    fn should_not_match() {
        assert_eq!(
            RouteMappingPath::from("/hello/world").matches(&"/hello/".into()),
            NotMatching
        );
        assert_eq!(
            RouteMappingPath::from("/hello").matches(&"/hello/world".into()),
            NotMatching
        );
        assert_eq!(
            RouteMappingPath::from("/api/booking/{booking_id}").matches(&"/api/booking/".into()),
            NotMatching
        );
    }

    #[test]
    fn should_match_variable() {
        assert_eq!(
            RouteMappingPath::from("/hello/{id}").matches(&"/hello/tristan".into()),
            MatchingWithVariables(HashMap::from([("id".to_string(), "tristan".to_string())]))
        );
        assert_eq!(
            RouteMappingPath::from("/api/booking/{booking_id}/passenger/{passenger_id}").matches(&"/api/booking/1/passenger/67".into()),
            MatchingWithVariables(HashMap::from([
                ("booking_id".to_string(), "1".to_string()),
                ("passenger_id".to_string(), "67".to_string())
            ]))
        );
    }
}