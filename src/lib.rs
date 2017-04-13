#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate reqwest;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate serde;
extern crate url;

pub mod api;
mod error;
mod method;
pub mod request;

use reqwest::Client;
use reqwest::header::{Authorization, Bearer};
use reqwest::StatusCode;
use reqwest::{RequestBuilder, Response};
use request::Request;
use method::{Get, Post, Delete};
pub use error::{Error, Result};
use url::Url;
use serde::Deserialize;


const STATIC_URL_ERROR: &'static str = "Base DigitalOcean URL is malformed.";
lazy_static! {
    static ref ROOT_URL: Url = Url::parse("https://api.digitalocean.com/v2")
        .expect(STATIC_URL_ERROR);
}

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

    fn get<T, O>(&self, request: Request<Get, O>) -> Result<T>
    where T: Deserialize {
        let req = self.client.get(request.url.clone());

        let mut response = self.fetch(req)?;
        
        match *response.status() {
            // Successes
            StatusCode::Ok => (), // Get success
            // Errors
            e => Err(Error::UnexpectedStatus(e))?,
        };
        
        let deserialized = response.json()?;
        Ok(deserialized)
    }

    // Delete requests do not return content.
    fn delete<O>(&self, request: Request<Delete, O>) -> Result<()> {
        let req = self.client.delete(request.url.clone());

        let req = match request.body {
            Some(v) => req.json(&v),
            None => req,
        };

        let response = self.fetch(req)?;

        match *response.status() {
            // Successes
            StatusCode::NoContent => (), // Delete success
            // Errors
            e => Err(Error::UnexpectedStatus(e))?,
        };

        Ok(())
    }

    fn post<T, O>(&self, request: Request<Post, O>) -> Result<T>
    where T: Deserialize {
        let req = self.client.post(request.url.clone());

        let req = match request.body {
            Some(v) => req.json(&v),
            None => req,
        };

        let mut response = self.fetch(req)?;

        match *response.status() {
            // Successes
            StatusCode::Created => (), // Post Success
            // Errors
            StatusCode::UnprocessableEntity => Err(Error::UnprocessableEntity)?,
            e => Err(Error::UnexpectedStatus(e))?,
        };

        let deserialized = response.json()?;
        Ok(deserialized)
    }

    fn fetch(&self, dispatch: RequestBuilder) -> Result<Response> {
        let response = dispatch
            .header(Authorization(Bearer {
                token: self.token.clone(),
            })).send()?;
        
        info!("Fetch status: {:?}", response.status());
        Ok(response)
    }
}

pub trait Retrievable<T>: Sized {
    fn retrieve(self, instance: &DigitalOcean) -> Result<T>;
}