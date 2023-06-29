use std::collections::HashMap;

#[derive(Debug)]
pub enum Method {
    PUT,
    GET,
    POST,
    PATCH,
    TRACE,
    OPTION,
    DELETE,
    CONNECT,
}

#[derive(Debug)]
pub enum Version {
    H1,
    H2,
    H3,
}

pub type Header<'buff> = HashMap<&'buff str, &'buff str>;
pub type Params<'buff> = HashMap<&'buff str, &'buff str>;

#[derive(Debug)]
/// Request struct provide eseay access to request data
pub struct Request<'buff> {
    pub headers: Header<'buff>,
    pub path: &'buff str,
    pub url: &'buff str,
    pub version: Version,
    pub params: Option<Params<'buff>>,
    pub method: Method,
    pub body: &'buff str,
}

impl Method {
    /// TODO: add doc.
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn buff_to_method(buff: &[u8]) -> Self {
        match buff {
            b"PUT" => Self::PUT,
            b"GET" => Self::GET,
            b"POST" => Self::POST,
            b"PATCH" => Self::PATCH,
            b"TRACE" => Self::TRACE,
            b"OPTION" => Self::OPTION,
            b"CONNECT" => Self::CONNECT,
            b"DELETE" => Self::DELETE,
            _ => todo!(),
        }
    }
}

impl Version {
    pub fn buff_to_version(buff: &[u8]) -> Self {
        match buff {
            b"HTTP/1.1" => Self::H1,
            b"HTTP/2" => Self::H2,
            b"HTTP/3" => Self::H3,
            _ => todo!(),
        }
    }
}

impl<'buff> Request<'buff> {
    /// TODO: Add Doc
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn new() -> Self {
        Self {
            headers: todo!(),
            path: todo!(),
            url: todo!(),
            params: todo!(),
            method: todo!(),
            body: todo!(),
            version: todo!(),
        }
    }
}
