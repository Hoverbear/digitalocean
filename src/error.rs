use reqwest;
use std;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// The reqest's API key is invalid or not authorized to view this resource.
    Unauthorized,
    /// The item exists (possibly on another account) or is otherwise unprocessable.
    UnprocessableEntity,
    /// Unable to fetch subresource because the Client is not set.
    MissingClient,
    /// An error originating from the Reqwest library.
    ReqwestError(reqwest::Error),
    /// An error originating from the URL handling in the Reqwest library.
    ReqwestUrlError(reqwest::UrlError),
    /// An unexpected status code was returned from the API. Please raise a ticket.
    UnexpectedStatus(reqwest::StatusCode),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ReqwestError(error)
    }
}

impl From<reqwest::UrlError> for Error {
    fn from(error: reqwest::UrlError) -> Self {
        Error::ReqwestUrlError(error)
    }
}