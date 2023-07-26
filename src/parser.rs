use std::collections::HashMap;

use crate::errors::ParsingError;

#[derive(Debug, PartialEq)]
pub enum HTTPMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    TRACE,
    PATCH,
}

impl HTTPMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GET => "GET",
            Self::HEAD => "HEAD",
            Self::POST => "POST",
            Self::PUT => "PUT",
            Self::DELETE => "DELETE",
            Self::CONNECT => "CONNECT",
            Self::TRACE => "TRACE",
            Self::PATCH => "PATCH",
        }
    }

    pub fn from(text: impl Into<String>) -> Option<Self> {
        match text.into().as_str() {
            "GET" => Some(Self::GET),
            "HEAD" => Some(Self::HEAD),
            "POST" => Some(Self::POST),
            "PUT" => Some(Self::PUT),
            "DELETE" => Some(Self::DELETE),
            "CONNECT" => Some(Self::CONNECT),
            "TRACE" => Some(Self::TRACE),
            "PATCH" => Some(Self::PATCH),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub method: HTTPMethod,
    pub route: String,
    pub http_version: f32,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn from(text: &str) -> Result<Self, ParsingError> {
        let lines: Vec<&str> = text.lines().collect();
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut body = String::new();

        if lines.is_empty() {
            return Err(ParsingError::new("Request is empty"));
        }

        // POST / HTTP/1.1
        let line_1: Vec<&str> = lines.first().unwrap().split_whitespace().collect();

        if line_1.len() < 2 {
            return Err(ParsingError::new(
                "First line of request doesn't have enought params",
            ));
        }

        // Extracts HTTP method
        let method = HTTPMethod::from(<&str>::clone(line_1.first().unwrap()));
        if method.is_none() {
            return Err(ParsingError::new("Failed to extract HTTP method"));
        }
        let method = method.unwrap();

        // Extracts route
        let route = line_1[1].to_string();
        if !route.contains('/') {
            return Err(ParsingError::new("Route doesn't contains /"));
        }

        // Extracts HTTP version
        let http_version = line_1[2];
        let http_version: Vec<&str> = http_version.split("/").collect();
        if http_version.len() != 2 {
            return Err(ParsingError::new("Failed to extract HTTP version"));
        }
        let http_version = http_version[1];
        let http_version = http_version.parse::<f32>();
        if http_version.is_err() {
            return Err(ParsingError::new("Failed to parse HTTP version"));
        }
        let http_version = http_version.unwrap();

        // Extract body and headers
        let mut read_body = false;
        for (i, line) in lines.iter().enumerate() {
            if read_body {
                body.push_str(line);
                continue;
            }

            if i == 0 {
                continue;
            }

            if line.is_empty() {
                read_body = true;
                continue;
            }

            let header: Vec<&str> = line.split(':').collect();
            if header.len() != 2 {
                return Err(ParsingError::new("Failed to parse the header"));
            }
            headers.insert(header[0].to_string(), header[1].to_string());
        }

        // Constructing the Request
        return Ok(Request {
            method,
            route,
            http_version,
            headers,
            body,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_method_method_from() {
        assert_eq!(HTTPMethod::GET, HTTPMethod::from("GET").unwrap());
        assert!(HTTPMethod::from("Not a HTTP method").is_none());
    }

    #[test]
    fn test_http_method_method_as_str() {
        let method = HTTPMethod::GET;
        assert_eq!("GET", method.as_str());
        assert_ne!("POST", method.as_str());
    }

    #[test]
    fn test_request_method_from() {
        let req = r#"POST /api/users/create HTTP/1.1
                    Host: example.com
                    Content-Type: application/json
                    Authorization: Bearer YOUR_ACCESS_TOKEN

                    {
                      "username": "john_doe",
                      "email": "john.doe@example.com",
                      "age": 30
                    }"#;

        match Request::from(req) {
            Ok(_) => assert!(true),
            Err(e) => assert!(false, "{e}"),
        }
    }
}
