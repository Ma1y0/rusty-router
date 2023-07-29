#[derive(Debug, PartialEq)]
pub enum StatusCode {
    // Informational 1xx
    Continue,
    SwitchingProtocols,
    Processing,

    // Successful 2xx
    Ok,
    Created,
    Accepted,
    NoContent,

    // Redirection 3xx
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,

    // Client Errors 4xx
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    UriTooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,

    // Server Errors 5xx
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HttpVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired,
}

impl StatusCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            // Informational 1xx
            StatusCode::Continue => "100 Continue",
            StatusCode::SwitchingProtocols => "101 Switching Protocols",
            StatusCode::Processing => "102 Processing",

            // Successful 2xx
            StatusCode::Ok => "200 OK",
            StatusCode::Created => "201 Created",
            StatusCode::Accepted => "202 Accepted",
            StatusCode::NoContent => "204 No Content",

            // Redirection 3xx
            StatusCode::MultipleChoices => "300 Multiple Choices",
            StatusCode::MovedPermanently => "301 Moved Permanently",
            StatusCode::Found => "302 Found",
            StatusCode::SeeOther => "303 See Other",

            // Client Errors 4xx
            StatusCode::BadRequest => "400 Bad Request",
            StatusCode::Unauthorized => "401 Unauthorized",
            StatusCode::PaymentRequired => "402 Payment Required",
            StatusCode::Forbidden => "403 Forbidden",
            StatusCode::NotFound => "404 Not Found",
            StatusCode::MethodNotAllowed => "405 Method Not Allowed",
            StatusCode::NotAcceptable => "406 Not Acceptable",
            StatusCode::ProxyAuthenticationRequired => "407 Proxy Authentication Required",
            StatusCode::RequestTimeout => "408 Request Timeout",
            StatusCode::Conflict => "409 Conflict",
            StatusCode::Gone => "410 Gone",
            StatusCode::LengthRequired => "411 Length Required",
            StatusCode::PreconditionFailed => "412 Precondition Failed",
            StatusCode::PayloadTooLarge => "413 Payload Too Large",
            StatusCode::UriTooLong => "414 URI Too Long",
            StatusCode::UnsupportedMediaType => "415 Unsupported Media Type",
            StatusCode::RangeNotSatisfiable => "416 Range Not Satisfiable",
            StatusCode::ExpectationFailed => "417 Expectation Failed",
            StatusCode::ImATeapot => "418 I'm a teapot",
            StatusCode::MisdirectedRequest => "421 Misdirected Request",
            StatusCode::UnprocessableEntity => "422 Unprocessable Entity",
            StatusCode::Locked => "423 Locked",
            StatusCode::FailedDependency => "424 Failed Dependency",
            StatusCode::TooEarly => "425 Too Early",
            StatusCode::UpgradeRequired => "426 Upgrade Required",
            StatusCode::PreconditionRequired => "428 Precondition Required",
            StatusCode::TooManyRequests => "429 Too Many Requests",
            StatusCode::RequestHeaderFieldsTooLarge => "431 Request Header Fields Too Large",
            StatusCode::UnavailableForLegalReasons => "451 Unavailable For Legal Reasons",

            // Server Errors 5xx
            StatusCode::InternalServerError => "500 Internal Server Error",
            StatusCode::NotImplemented => "501 Not Implemented",
            StatusCode::BadGateway => "502 Bad Gateway",
            StatusCode::ServiceUnavailable => "503 Service Unavailable",
            StatusCode::GatewayTimeout => "504 Gateway Timeout",
            StatusCode::HttpVersionNotSupported => "505 HTTP Version Not Supported",
            StatusCode::VariantAlsoNegotiates => "506 Variant Also Negotiates",
            StatusCode::InsufficientStorage => "507 Insufficient Storage",
            StatusCode::LoopDetected => "508 Loop Detected",
            StatusCode::NotExtended => "510 Not Extended",
            StatusCode::NetworkAuthenticationRequired => "511 Network Authentication Required",
        }
    }

    pub fn from(code: impl Into<i32>) -> Option<Self> {
        let code = code.into();
        match code {
            // Informational 1xx
            100 => Some(Self::Continue),
            101 => Some(Self::SwitchingProtocols),
            102 => Some(Self::Processing),

            // Successful 2xx
            200 => Some(StatusCode::Ok),
            201 => Some(Self::Created),
            202 => Some(Self::Accepted),
            204 => Some(Self::NoContent),

            // Redirection 3xx
            300 => Some(Self::MultipleChoices),
            301 => Some(Self::MovedPermanently),
            302 => Some(Self::Found),
            303 => Some(Self::SeeOther),

            // Cient Errors 4xx
            400 => Some(Self::BadRequest),
            401 => Some(Self::Unauthorized),
            402 => Some(Self::Unauthorized),
            403 => Some(Self::Forbidden),
            404 => Some(Self::NotFound),
            405 => Some(Self::MethodNotAllowed),
            406 => Some(Self::NotAcceptable),
            407 => Some(Self::ProxyAuthenticationRequired),
            408 => Some(Self::RequestTimeout),
            409 => Some(Self::Conflict),
            410 => Some(Self::Gone),
            411 => Some(Self::LengthRequired),
            412 => Some(Self::PreconditionRequired),
            413 => Some(Self::PayloadTooLarge),
            414 => Some(Self::UriTooLong),
            415 => Some(Self::UnsupportedMediaType),
            416 => Some(Self::RangeNotSatisfiable),
            417 => Some(Self::ExpectationFailed),
            418 => Some(Self::ImATeapot),
            421 => Some(Self::MisdirectedRequest),
            422 => Some(Self::UnprocessableEntity),
            423 => Some(Self::Locked),
            424 => Some(Self::FailedDependency),
            425 => Some(Self::TooEarly),
            426 => Some(Self::UpgradeRequired),
            428 => Some(Self::PreconditionRequired),
            429 => Some(Self::TooManyRequests),
            431 => Some(Self::RequestHeaderFieldsTooLarge),
            432 => Some(Self::Locked),
            451 => Some(Self::UnavailableForLegalReasons),

            // Server Errors 5xx
            500 => Some(Self::InternalServerError),
            501 => Some(Self::NotImplemented),
            502 => Some(Self::BadGateway),
            503 => Some(Self::ServiceUnavailable),
            504 => Some(Self::GatewayTimeout),
            505 => Some(Self::HttpVersionNotSupported),
            506 => Some(Self::VariantAlsoNegotiates),
            507 => Some(Self::InsufficientStorage),
            508 => Some(Self::LoopDetected),
            510 => Some(Self::NotExtended),
            511 => Some(Self::NetworkAuthenticationRequired),

            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_code_method_as_str() {
        let method = StatusCode::Ok;
        assert_eq!("200 OK", method.as_str());
    }

    #[test]
    fn test_status_code_method_from() {
        match StatusCode::from(500) {
            Some(a) => assert_eq!(StatusCode::InternalServerError, a),
            None => assert!(false, "Invalid status code"),
        }
    }
}
