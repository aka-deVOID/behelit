use std::default;

#[derive(Debug)]
pub enum Status {
    // 100
    Continue,
    SwitchingProtocols,
    Processing,
    EarlyHints,
    // 200
    Ok,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    IMUsed,
    // 300
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    TemporaryRedirect,
    PermanentRedirect,
    // 400
    BadRequest,
    Unauthorized,
    PaymentRequired,
    NotFound,
    Forbidden,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    URITooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    Imateapot,
    MisdirectedRequest,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,
    // 500
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    HTTPVersionNotSupported,
    VariantAlsoNegotiates,
    NotExtended,
    NetworkAuthenticationRequired,
}

trait Responder {
    fn build(&self) {}
}

#[derive(Debug)]
pub struct Response {
    pub status: Status,
    pub body: String,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            status: Status::Ok,
            body: "".to_string(),
        }
    }
}

impl Responder for Response {
    fn build(&self) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn response_test() {}
}
