//! Errors with semantic, crate level meaning.
//!
//! This crate uses [`failure`](https://github.com/withoutboats/failure) to accomplish error handling.
//!
//! While functions that return `Error` could return errors from other libraries, the errors in
//! `ErrorKind` have special semantic meaning to this crate.

use reqwest;
use serde_json;

/// Errors which have crate specific meanings.
#[derive(Debug, Fail)]
pub enum ErrorKind {
    /// The item does not exist or otherwise cannot be found.
    #[fail(display = "Not Found")]
    NotFound,
    /// The reqest's API key is invalid or not authorized to view this resource.
    #[fail(display = "Unauthorized")]
    Unauthorized,
    /// The specified `Host` header value is invalid.
    #[fail(display = "Invalid hostname")]
    InvalidHostHeader,
    /// The specified `User-Agent` header value is invalid.
    #[fail(display = "Invalid user agent")]
    InvalidUserAgent,
    /// An unexpected status code was returned from the API. Please raise a ticket.
    #[fail(display = "Unexpected status code: {}", _0)]
    UnexpectedStatus(reqwest::StatusCode),
    /// The item exists (possibly on another account), the limit on this item has been reached,
    /// or this request is otherwise unprocessable.
    #[fail(display = "Unprocessable entity: {}", _0)]
    UnprocessableEntity(serde_json::Value),
}
