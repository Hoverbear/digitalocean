//! Crate level docs.

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate reqwest;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate serde;
extern crate url_serde;
extern crate url;
extern crate chrono;
#[macro_use] extern crate error_chain;

pub mod api;
mod error;
pub mod method;
pub mod request;
mod client;

use error::*;
pub use error::{Error, ErrorKind};

use request::{Request, Executable};
use method::Method;
use api::{HasResponse};
use url::Url;

const STATIC_URL_ERROR: &'static str = "Base DigitalOcean URL is malformed.";
lazy_static! {
    static ref ROOT_URL: Url = Url::parse("https://api.digitalocean.com/v2")
        .expect(STATIC_URL_ERROR);
}

/// A DigitalOcean Client that holds an API key.
#[derive(Clone)]
pub struct DigitalOcean {
    client: client::Client,
    token: String,
}

impl DigitalOcean {
    /// Create a DigitalOcean client with the given API key.
    pub fn new<T: Into<String>>(token: T) -> Result<Self> {
        info!("Created.");
        Ok(DigitalOcean {
            client: client::Client::new()?,
            token: token.into(),
        })
    }

    pub fn execute<A,V>(&self, request: Request<A,V>) -> Result<V>
    where A: Method, 
          Request<A,V>: Executable<V>,
          V: HasResponse {
        request.execute(self)
    }
}
