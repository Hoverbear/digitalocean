use reqwest;
use serde_json;
use serde_json::Value;
use url;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        SerdeJson(serde_json::Error);
        UrlParse(url::ParseError);
    }

    // Define additional `ErrorKind` variants. The syntax here is
    // the same as `quick_error!`, but the `from()` and `cause()`
    // syntax is not supported.
    errors {
        /// The reqest's API key is invalid or not authorized to view this resource.
        Unauthorized {
            description("Unauthorized")
            display("Unauthorized")
        }
        /// The item exists (possibly on another account), the limit on this item has been reached,
        /// or this request is otherwise unprocessable.
        UnprocessableEntity(t: Value) {
            description("Unprocessable entity")
            display("Unprocessable entity: {}", t)
        }
        /// An unexpected status code was returned from the API. Please raise a ticket.
        UnexpectedStatus(t: reqwest::StatusCode) {
            description("Unexpected status code")
            display("Unexpected status code: {}", t)
        }
        /// The item does not exist or otherwise cannot be found.
        NotFound {
            description("Not found")
            display("Not found")
        }
    }
}
