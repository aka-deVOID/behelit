use std::str::from_utf8_unchecked;

use crate::error::Result;
use crate::request::{Header, Method, Params, Request, Version};

trait Parser {
    fn header(buff: &[u8]);

    fn url_params(buff: &[u8]) -> (&str, Option<Params>);

    fn request_line(line: &[u8]) -> Result<(Method, &str, Option<Params>, Version)>;
    fn param_to_hashmap(buff: &[u8]);
    fn parse(buff: &[u8]) -> Self;
}

impl<'buff> Parser for Request<'buff> {
    fn header(buff: &[u8]) {
        todo!()
    }

    fn param_to_hashmap(buff: &[u8]) {
        let a = &buff[1..buff.len()];

        todo!()
    }

    fn url_params(buff: &[u8]) -> (&str, Option<Params>) {
        let res = if let Ok(indx) = buff.binary_search(&b'?') {
            let (url, params) = buff.split_at(indx);
            unsafe {
                println!(
                    "{:?}, {:?}",
                    from_utf8_unchecked(url),
                    from_utf8_unchecked(params)
                );
                return (unsafe { from_utf8_unchecked(url) }, None);
            };
        } else {
            return (unsafe { from_utf8_unchecked(buff) }, None);
        };
    }

    fn request_line(line: &[u8]) -> Result<(Method, &str, Option<Params>, Version)> {
        let mut method: Method = Method::GET;
        let mut url: &str = "";
        let mut params: Option<Params> = None;
        let mut version: Version = Version::H1;
        for (index, part) in line.split(|x| *x == 32).enumerate() {
            match index {
                0 => method = Method::buff_to_method(part),    // Method,
                1 => (url, params) = Self::url_params(part),   // url + params
                2 => version = Version::buff_to_version(part), // HTTP version
                _ => todo!(),                                  // error
            }
        }
        Ok((method, url, params, version))
    }

    fn parse(buff: &[u8]) -> Self {
        for (num, line) in buff.split(|x| *x == b'\n').enumerate() {
            println!("{:?}, {:?}", line, num);
            if num == 0 {
                let a = Self::request_line(line).expect("err");
                println!("{:?}", a);
                continue;
            } else if line.is_empty() {
                println!("{}", num);
                break;
            }
        }
        Self {
            headers: Header::new(),
            path: "",
            url: "",
            version: Version::H1,
            params: None,
            method: Method::GET,
            body: "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let raw_request = r#"POST /?AuthId=YOURKEY&Action=WebServiceAction&Signature=rcLXfkPldrYm04 HTTP/1.1
Content-Type: text/tab-separated-values; charset=iso-8859-1
Content-Length: []
Host: webservices.domain.com
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Encoding: aidentity
User-Agent: Mozilla/3.0 (compatible; Indy Library)

name    id
John    G12N
Sarah   J87M
Bob     N33Y"#;
        let c = Request::parse(raw_request.as_bytes());
    }
}
