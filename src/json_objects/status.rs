use extism_pdk::{ToBytes, Json};
use serde::Serialize;

/// An HTTP response status code.
#[derive(Serialize, ToBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
#[serde(untagged)]
#[repr(u16)]
pub enum Status {
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,

    /// HTTP response [`Status`] for when the server handled the request without error.
    Ok = 200,

    /// HTTP response [`Status`] for when the server created a new resource in response to the request.
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    ImUsed = 226,

    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    /// HTTP response [`Status`] for when the client made a malformed request.
    BadRequest = 400,

    /// HTTP response [`Status`] for when a resource requires authorization to be accessed.
    Unauthorized = 401,
    PaymentRequired = 402,

    /// HTTP response [`Status`] for when a resource is not permitted to be accessed by a certain user.
    Forbidden = 403,

    /// HTTP response [`Status`] for when a resource cannot be found.
    NotFound = 404,

    /// HTTP response [`Status`] for when the wrong method is used on an otherwise correct path.
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,

    /// HTTP response [`Status`] for when a resource that once existed no longer exists.
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    ContentTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,

    /// HTTP response [`Status`] for when the client attempts to brew coffee in a teapot.
    ImATeapot = 418,
    MisdirectedRequest = 421,
    UnprocessableContent = 422,
    Locked = 423,
    FailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,

    /// HTTP response [`Status`] for when there's an error on the server side.
    InternalServerError = 500,

    /// HTTP response [`Status`] for when code on the server side hasn't been implemented yet.
    NotImplemented = 501,

    /// HTTP response [`Status`] for when there's an issue with the reverse proxy.
    BadGateway = 502,
    ServiceUnavailable = 503,

    /// HTTP response [`Status`] for when the reverse proxy decides a request is taking too long and it should be dropped.
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,

    /// For when using a status code that doesn't exist in the HTTP protocol standards.
    Custom(u16)
}

impl From<u16> for Status {
    fn from(status: u16) -> Self {
        match status {
            100 => Status::Continue,
            101 => Status::SwitchingProtocols,
            102 => Status::Processing,
            103 => Status::EarlyHints,

            200 => Status::Ok,
            201 => Status::Created,
            202 => Status::Accepted,
            203 => Status::NonAuthoritativeInformation,
            204 => Status::NoContent,
            205 => Status::ResetContent,
            206 => Status::PartialContent,
            207 => Status::MultiStatus,
            208 => Status::AlreadyReported,
            226 => Status::ImUsed,

            300 => Status::MultipleChoices,
            301 => Status::MovedPermanently,
            302 => Status::Found,
            303 => Status::SeeOther,
            304 => Status::NotModified,
            307 => Status::TemporaryRedirect,
            308 => Status::PermanentRedirect,

            400 => Status::BadRequest,
            401 => Status::Unauthorized,
            402 => Status::PaymentRequired,
            403 => Status::Forbidden,
            404 => Status::NotFound,
            405 => Status::MethodNotAllowed,
            406 => Status::NotAcceptable,
            407 => Status::ProxyAuthenticationRequired,
            408 => Status::RequestTimeout,
            409 => Status::Conflict,
            410 => Status::Gone,
            411 => Status::LengthRequired,
            412 => Status::PreconditionFailed,
            413 => Status::ContentTooLarge,
            414 => Status::UriTooLong,
            415 => Status::UnsupportedMediaType,
            416 => Status::RangeNotSatisfiable,
            417 => Status::ExpectationFailed,
            418 => Status::ImATeapot,
            421 => Status::MisdirectedRequest,
            422 => Status::UnprocessableContent,
            423 => Status::Locked,
            424 => Status::FailedDependency,
            425 => Status::TooEarly,
            426 => Status::UpgradeRequired,
            428 => Status::PreconditionRequired,
            429 => Status::TooManyRequests,
            431 => Status::RequestHeaderFieldsTooLarge,
            451 => Status::UnavailableForLegalReasons,

            500 => Status::InternalServerError,
            501 => Status::NotImplemented,
            502 => Status::BadGateway,
            503 => Status::ServiceUnavailable,
            504 => Status::GatewayTimeout,
            505 => Status::HttpVersionNotSupported,
            506 => Status::VariantAlsoNegotiates,
            507 => Status::InsufficientStorage,
            508 => Status::LoopDetected,
            510 => Status::NotExtended,
            511 => Status::NetworkAuthenticationRequired,

            status => Status::Custom(status)
        }
    }
}