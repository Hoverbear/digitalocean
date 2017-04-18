use reqwest;
use std;
use url;
use serde_json;

use serde_json::Value;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// The reqest's API key is invalid or not authorized to view this resource.
    Unauthorized,
    /// The item exists (possibly on another account), the limit on this item has been reached,
    /// or this request is otherwise unprocessable.
    UnprocessableEntity(Value),
    /// Unable to fetch subresource because the Client is not set.
    MissingClient,
    /// An error originating from the Reqwest library.
    ReqwestError(reqwest::Error),
    /// An unexpected status code was returned from the API. Please raise a ticket.
    UnexpectedStatus(reqwest::StatusCode),
    /// An error originating from serde_json.
    SerdeJsonError(serde_json::Error),
    /// A parse error from the url crate.
    UrlParseError(url::ParseError),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ReqwestError(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::SerdeJsonError(error)
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Error::UrlParseError(error)
    }
}