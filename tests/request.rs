use behelit::request::*;

#[test]
fn method_parser_test() {
    todo!()
}

#[test]
fn request_parser() {
    let mut request = Request::new();
}

#[test]
fn prerequest_builder_test() {
    let req = PreRequest::new();
    req.body(&[12, 33, 44, 55]);
}
