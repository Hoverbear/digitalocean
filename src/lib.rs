#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate reqwest;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate serde;

pub mod api;
pub mod types;
mod error;
mod client;

use reqwest::Url;
use api::{Images, Regions, Domains};
use client::Client;
pub use error::{Error, Result};

// macro_rules! api_call { ($e:expr) => (concat!("https://api.digitalocean.com/v2/", $e)) }

lazy_static! {
    static ref ROOT_URL: Url = Url::parse("https://api.digitalocean.com/v2/")
        .expect("This URL is static and should be well formed.");
}

#[derive(Clone)]
pub struct DigitalOcean {
    client: Client,
}

impl DigitalOcean {
    /// Create a DigitalOcean client with the given API key.
    pub fn new<T: Into<String>>(token: T) -> Result<Self> {
        info!("Created.");
        Ok(DigitalOcean {
            client: Client::new(token.into())?,
        })
    }

    /// Access image related API calls.
    pub fn images(&self) -> Images {
        Images::new(&self.client)
    }

    /// Access region related API calls.
    pub fn regions(&self) -> Regions {
        Regions::new(&self.client)
    }

    /// Access domain related API calls.
    pub fn domains(&self) -> Domains {
        Domains::new(&self.client)
    }
}

