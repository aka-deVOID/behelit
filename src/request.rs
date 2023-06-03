use crate::error::Result;
use std::{collections::HashMap, io::Bytes};

type Header<'header> = HashMap<&'header str, &'header str>;

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

#[derive(Debug)]
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
    pub fn new(self, request: String) -> Result<Self> {
        Ok(Self {
            headers: todo!(),
            path: todo!(),
            url: todo!(),
            method: todo!(),
            body: todo!(),
            params: todo!(),
        })
    }

    /// parse first line of HTTP request METHOD, URL, VERSION
    pub fn first_line_parser(line_one: &str) {
        let mut white_split = line_one.split_whitespace();
        let method = white_split.next();
    }

    pub fn parse(request: &[u8]) -> Result<Self> {
        todo!()
    }
}
