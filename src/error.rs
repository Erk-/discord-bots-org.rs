use serde_json::Error as JsonError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use url::ParseError as UrlParseError;

#[cfg(feature = "reqwest")]
use reqwest::{
    Error as ReqwestError,
    Response as ReqwestResponse,
    header::InvalidHeaderValue,
};

/// A result type to compose a successful value and the library's [`Error`]
/// type.
///
/// [`Error`]: enum.Error.html
pub type Result<T> = StdResult<T, Error>;

/// An error type to compose a singular error enum between various dependencies'
/// errors.
#[derive(Debug)]
pub enum Error {
    /// When a URL is invalid.
    InvalidUrl(UrlParseError),
    /// An error from the `serde_json` crate.
    ///
    /// A potential reason for this is when there is an error deserializing a
    /// JSON response body.
    Json(JsonError),
    /// An error from the `reqwest` crate when it is enabled.
    #[cfg(feature = "reqwest")]
    Reqwest(ReqwestError),
    /// An error indicating a bad request when using `reqwest`.
    #[cfg(feature = "reqwest")]
    ReqwestBad(Box<ReqwestResponse>),
    /// An error indicating that a header value was invalid.
    #[cfg(feature = "reqwest")]
    ReqwestHeaderValue(InvalidHeaderValue),
    /// An error indicating an invalid request when using `reqwest`.
    #[cfg(feature = "reqwest")]
    ReqwestInvalid(Box<ReqwestResponse>),
    /// An error indicating an unathorized request when using `reqwest`.
    #[cfg(feature = "reqwest")]
    ReqwestUnauthorized(Box<ReqwestResponse>),
}

#[cfg(feature = "reqwest")]
impl From<InvalidHeaderValue> for Error {
    fn from(err: InvalidHeaderValue) -> Error {
        Error::ReqwestHeaderValue(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::Json(err)
    }
}

#[cfg(feature = "reqwest")]
impl From<UrlParseError> for Error {
    fn from(err: UrlParseError) -> Self {
        Error::InvalidUrl(err)
    }
}

#[cfg(feature = "reqwest")]
impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Self {
        Error::Reqwest(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::InvalidUrl(e) => e.description(),
            Error::Json(e) => e.description(),
            #[cfg(feature = "reqwest")]
            Error::Reqwest(e) => e.description(),
            #[cfg(feature = "reqwest")]
            Error::ReqwestBad(_) => "Request bad",
            #[cfg(feature = "reqwest")]
            Error::ReqwestHeaderValue(e) => e.description(),
            #[cfg(feature = "reqwest")]
            Error::ReqwestInvalid(_) => "Request invalid",
            #[cfg(feature = "reqwest")]
            Error::ReqwestUnauthorized(_) => "Request auth bad",
        }
    }
}
