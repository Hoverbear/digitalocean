#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate reqwest;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate serde;
extern crate url;

pub mod api;
mod error;
mod request;

use reqwest::Client;
use reqwest::header::{Authorization, Bearer};
use reqwest::StatusCode;
use request::{Request, Method};
pub use error::{Error, Result};
use url::Url;
use std::io::Read;

#[derive(Clone)]
pub struct DigitalOcean {
    client: Client,
    token: String,
}

impl DigitalOcean {
    /// Create a DigitalOcean client with the given API key.
    pub fn new<T: Into<String>>(token: T) -> Result<Self> {
        info!("Created.");
        Ok(DigitalOcean {
            client: Client::new()?,
            token: token.into(),
        })
    }

    pub fn execute<T>(&self, request: &Request<T>) -> Result<T>
    where T: Parse {
        info!("Executing.");

        let url = request.url.clone();

        let dispatch = match request.method {
            Method::Get => self.client.get(url),
            Method::Post => self.client.post(url),
            Method::Delete => self.client.delete(url)
        };


        let dispatch = dispatch.json(&request.body);

        let response = dispatch
            .header(Authorization(Bearer {
                token: self.token.clone(),
            })).send()?;

        info!("{:?}", response.status());
        match *response.status() {
            StatusCode::UnprocessableEntity => Err(Error::UnprocessableEntity)?,
            _ => (),
        };

        let parsed = T::parse(response)?; // TODO: TryFrom?

        Ok(parsed)
    }
}

static STATIC_PARSE_ERROR: &'static str = "Failed to parse a staticly defined URL.";

lazy_static! {
    static ref ROOT_URL: Url = Url::parse("https://api.digitalocean.com/v2/")
        .expect(STATIC_PARSE_ERROR);
    static ref DOMAINS_URL: Url = (*ROOT_URL).join("domains/")
        .expect(STATIC_PARSE_ERROR);
}
    
pub trait Parse: Sized {
    fn parse<R>(reader: R) -> Result<Self> where R: Read;
}
