use crate::error::{Error, Result};
use std::collections::HashMap;

type Header<'header> = HashMap<&'header str, &'header str>;

#[derive(Debug, Default)]
pub enum Method {
    PUT,
    #[default]
    GET,
    POST,
    PATCH,
    TRACE,
    OPTION,
    DELETE,
    CONNECT,
}

impl Method {
    fn str_to_method(str: Option<&str>) -> Result<Self> {
        match str {
            Some("GET") => Ok(Self::GET),
            Some("POST") => Ok(Self::POST),
            Some("PUT") => Ok(Self::PUT),
            _ => todo!(),
        }
    }
}

#[derive(Debug, Default)]
/// Request struct provide eseay access to request data
pub struct Request<'buf, 'header> {
    pub headers: Header<'header>,
    pub path: &'buf str,
    pub url: &'buf str,
    pub params: HashMap<&'buf str, &'buf str>,
    pub method: Method,
    pub body: &'buf str,
}

impl<'buf, 'header> Request<'buf, 'header> {
    pub fn new(self) -> Result<Self> {
        Ok(Self::default())
    }
    /// parse first line of HTTP request METHOD, URL, VERSION
    pub fn first_line_parser(line_one: &str) -> Result<(&str, &str, &str)> {
        let mut line_iter = line_one.split_whitespace();
        let method = Method::str_to_method(line_iter.next());
        let path = line_iter.next();
        let version = line_iter.next();

        todo!()
    }

    pub fn parse(request: &[u8]) -> Result<Self> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_line_parser_test() {
        Request::first_line_parser("GET / HTTP/1.1");
    }
}
