//! Errors with semantic, crate level meaning.
//!
//! This crate uses [`thiserror`](https://github.com/dtolnay/thiserror) to accomplish error handling.
//!
//! While functions that return `Error` could return errors from other libraries, the errors in
//! `ErrorKind` have special semantic meaning to this crate.

use reqwest;
use serde_json;

/// Errors which have crate specific meanings.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The item does not exist or otherwise cannot be found.
    #[error("Not Found")]
    NotFound,
    /// The reqest's API key is invalid or not authorized to view this resource.
    #[error("Unauthorized")]
    Unauthorized,
    /// An unexpected status code was returned from the API. Please raise a ticket.
    #[error("Unexpected status code: {0}")]
    UnexpectedStatus(reqwest::StatusCode),
    /// The item exists (possibly on another account), the limit on this item has been reached,
    /// or this request is otherwise unprocessable.
    #[error("Unprocessable entity: {0}")]
    UnprocessableEntity(serde_json::Value),
    /// There was a miscellaneous error processing the request. Please look at the documentation of
    /// `reqwest` to learn more about how to handle these errors.
    #[error("{0}")]
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::ReqwestError(err)
    }
}