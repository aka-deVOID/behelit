use crate::error::Result;
use std::{collections::HashMap, str::from_utf8};

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

type Header<'buf> = HashMap<&'buf str, &'buf str>;
type Params<'buf> = HashMap<&'buf str, &'buf str>;

#[derive(Debug, Default)]
/// Request struct provide eseay access to request data
pub struct Request<'buf> {
    pub headers: Header<'buf>,
    pub path: &'buf str,
    pub url: &'buf str,
    pub params: Params<'buf>,
    pub method: Method,
    pub body: &'buf str,
}

#[derive(Debug, Default)]
/// PreRequest Builder struct for Request
pub struct PreRequest<'buf> {
    pub headers: Header<'buf>,
    pub path: &'buf [u8],
    pub url: &'buf [u8],
    pub params: Params<'buf>,
    pub method: Method,
    pub body: &'buf [u8],
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

/// Build Request
pub trait RequestBuilder<'buf> {
    /// TODO: add doc
    fn headers(self, headers: Header) -> Self;
    /// TODO: add doc.
    fn url(self, url: &[u8]) -> Self;
    /// TODO: add doc.
    fn params(self, params: Params) -> Self;
    /// TODO: add doc.
    fn method(self, method: &[u8]) -> Self;
    /// TODO: add doc.
    fn body(self, body: &[u8]) -> Self;
    /// TODO: add doc.
    fn build(self) -> Result<Request<'buf>>;
}

impl<'buf> PreRequest<'buf> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'buf> RequestBuilder<'buf> for PreRequest<'buf> {
    fn headers(self, headers: Header) -> Self {
        self
    }

    fn url(self, url: &[u8]) -> Self {
        self
    }

    fn params(self, params: Params) -> Self {
        self
    }

    fn method(self, method: &[u8]) -> Self {
        self
    }

    fn body(self, body: &[u8]) -> Self {
        self
    }

    fn build(self) -> Result<Request<'buf>> {
        Ok(Request {
            headers: self.headers,
            path: from_utf8(self.path)?,
            url: from_utf8(self.path)?,
            params: self.params,
            method: self.method,
            body: from_utf8(self.body)?,
        })
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
}
