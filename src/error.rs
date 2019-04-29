//! Errors with semantic, crate level meaning.
//!
//! This crate uses [`failure`](https://github.com/withoutboats/failure) to accomplish error handling.
//!
//! While functions that return `Error` could return errors from other libraries, the errors in
//! `ErrorKind` have special semantic meaning to this crate.

#[cfg(feature = "spaces")]
use crate::api::SpacesError;

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
    /// An unexpected error has occurred in HTTP.
    #[fail(display = "HTTP error has occurred: {}", _0)]
    #[cfg(feature = "spaces")]
    Http(hyper::Error),
    /// An unexpected status code was returned from the API. Please raise a ticket.
    #[fail(display = "Unexpected status code: {}", _0)]
    UnexpectedStatus(reqwest::StatusCode),
    /// The item exists (possibly on another account), the limit on this item has been reached,
    /// or this request is otherwise unprocessable.
    #[fail(display = "Unprocessable entity: {}", _0)]
    UnprocessableEntity(serde_json::Value),
    /// An error has occurred in parsing XML response from the API server.
    #[fail(display = "Unable to parse XML response: {}", _0)]
    #[cfg(feature = "spaces")]
    Xml(XmlError),
    // FIXME: Support fine-grained errors for Spaces API based on error codes.
    /// Spaces API has returned an error response.
    #[fail(display = "Error from Spaces API: {}", _0)]
    #[cfg(feature = "spaces")]
    Spaces(SpacesError),
}

/* Helper impls */

#[cfg(feature = "spaces")]
mod spaces {
    use crate::api::SpacesError;

    use std::error::Error;
    use std::fmt::{self, Display};

    /// Wrapper type for XML error since `serde_xml_rs::Error` is `!Sync`
    /// and we need that bound for `failure`.
    #[derive(Debug)]
    pub struct XmlError(serde_xml_rs::ErrorKind);

    impl Display for SpacesError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Code {}", self.code)
        }
    }

    impl From<serde_xml_rs::Error> for XmlError {
        fn from(e: serde_xml_rs::Error) -> Self {
            XmlError(e.0)
        }
    }

    impl Display for XmlError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl Error for XmlError {}
}

#[cfg(feature = "spaces")]
pub use self::spaces::XmlError;
