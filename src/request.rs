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
    /// TODO: add doc.
    ///
    /// # Errors
    ///
    /// This function will return an error if .
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
pub struct Request<'buf> {
    pub headers: Header<'buf>,
    pub path: &'buf str,
    pub url: &'buf str,
    pub params: HashMap<&'buf str, &'buf str>,
    pub method: Method,
    pub body: &'buf str,
}

/// Build Request
trait RequestBuilder<'buf> {
    /// TODO: add doc
    fn headers();
    /// TODO: add doc.
    fn url();
    /// TODO: add doc.
    fn params();
    /// TODO: add doc.
    fn method();
    /// TODO: add doc.
    fn body();
    /// TODO: add doc.
    fn build() -> Request<'buf>;
}

impl<'buf> RequestBuilder<'buf> for Request<'buf> {
    fn headers() {
        todo!()
    }

    fn url() {
        todo!()
    }

    fn params() {
        todo!()
    }

    fn method() {
        todo!()
    }

    fn body() {
        todo!()
    }

    fn build() -> Request<'buf> {
        todo!()
    }
}

impl<'buf> Request<'buf> {
    /// TODO: Add Doc
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn new() -> Result<Self> {
        Ok(Self::default())
    }

    /// TODO: Move to Parser
    ///
    /// # Errors
    ///
    /// This function will return an error if .
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
