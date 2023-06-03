#![allow(unused, dead_code)]

mod core;
mod error;
mod parser;
mod request;
mod response;
mod server;

use std::collections::HashMap;
use thiserror::Error;
trait Api {
    // TODO async
    fn get(&self) {
        todo!()
    }

    fn post(&self) {
        todo!()
    }

    fn put(&self) {
        todo!()
    }

    fn patch(&self) {
        todo!()
    }

    fn delete(&self) {
        todo!()
    }
}

const MAX_LENGHT: usize = usize::MAX;
#[derive(Debug)]
struct App {
    pub main: String,
}

impl App {
    // fn new() -> Self {  }
}

#[cfg(test)]
mod tests {
    use std::{io::BufRead, str::Bytes};

    use tokio::io::AsyncBufReadExt;

    use super::*;

    #[test]
    fn test_request<'a>() {
        let request = r#"GET /hello/?hello=val HTTP/1.1
Authorization: Bearer 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
Content-Type: application/json
User-Agent: PostmanRuntime/7.31.1
Accept: */*
Postman-Token: 4d35c0de-049f-4383-880b-94a61f320a2c
Host: 127.0.0.1:1337
Accept-Encoding: gzip, deflate, br
Connection: keep-alive
Content-Length: 19
{
        "hello": "bye"
}"#;
    }

    #[test]
    fn test_status() {}
}
