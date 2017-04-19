//! Crate level docs.

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate reqwest;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate serde;
extern crate url_serde;
extern crate url;

pub mod api;
mod error;
pub mod action;
pub mod request;
pub mod values;

pub use error::{Error, Result};
pub use request::Retrievable;

use reqwest::Client;
use reqwest::header::{Authorization, Bearer};
use reqwest::StatusCode;
use reqwest::{RequestBuilder, Response};
use request::Request;
use action::{List, Get, Create, Delete, Update};
use api::{HasValue, HasPagination};
use values::HasResponse;
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

    fn get<V>(&self, request: Request<Get, V>) -> Result<V>
    where V: Deserialize + Clone + HasResponse,
          V::Response: HasValue<Value=V> {
        info!("GET {:?}", request.url);
        let req = self.client.get(request.url.clone());

        let mut response = self.fetch(req)?;
        
        match *response.status() {
            // Successes
            StatusCode::Ok => (), // Get success
            // Errors
            e => Err(Error::UnexpectedStatus(e))?,
        };
        
        let deserialized: V::Response = response.json()?;
        Ok(deserialized.value())
    }

    fn list<V>(&self, request: Request<List, Vec<V>>) -> Result<Vec<V>>
    where V: Deserialize + Clone,
          Vec<V>: HasResponse,
          <Vec<V> as HasResponse>::Response: HasValue<Value=Vec<V>> + HasPagination {
        info!("LIST {:?}", request.url);
        // This may be a paginated response. We need to buffer.
        let mut buffer = Vec::new();
        let mut current_url = request.url.clone();

        current_url.query_pairs_mut()
            .append_pair("per_page", &api::MAX_PER_PAGE.to_string());

        loop {
            let req = self.client.get(current_url.clone());
            let mut response = self.fetch(req)?;
            
            match *response.status() {
                // Successes
                StatusCode::Ok => (), // Get success
                // Errors
                e => Err(Error::UnexpectedStatus(e))?,
            };

            let deserialized: <Vec<V> as HasResponse>::Response = response.json()?;

            let next_page = deserialized.next_page();
            buffer.extend(deserialized.value());
            current_url = match next_page {
                Some(v) => v,
                None => break,
            };
            info!("Fetching next page...")
        }

        Ok(buffer)
    }

    // Delete requests do not return content.
    fn delete<V>(&self, request: Request<Delete, V>) -> Result<()> {
        info!("DELETE {:?}", request.url);
        let req = self.client.delete(request.url.clone());

        let response = self.fetch(req)?;

        match *response.status() {
            // Successes
            StatusCode::NoContent => (), // Delete success
            // Errors
            e => Err(Error::UnexpectedStatus(e))?,
        };

        Ok(())
    }

    fn post<V>(&self, request: Request<Create, V>) -> Result<V>
    where V: Deserialize + Clone + HasResponse,
          V::Response: HasValue<Value=V> {
        info!("POST {:?}", request.url);
        let req = self.client.post(request.url.clone());

        let req = req.json(&request.body.clone());

        let mut response = self.fetch(req)?;

        match *response.status() {
            // Successes
            StatusCode::Created => (), // Post Success
            // Errors
            StatusCode::UnprocessableEntity => Err(Error::UnprocessableEntity(response.json()?))?,
            e => Err(Error::UnexpectedStatus(e))?,
        };

        let deserialized: V::Response = response.json()?;
        Ok(deserialized.value())
    }

    fn put<V>(&self, request: Request<Update, V>) -> Result<V>
    where V: Deserialize + Clone + HasResponse,
          V::Response: HasValue<Value=V> {
        info!("PUT {:?}", request.url);
        let req = self.client.put(request.url.clone());

        let req = req.json(&request.body.clone());

        let mut response = self.fetch(req)?;

        match *response.status() {
            // Successes
            StatusCode::Ok => (), // Update success
            // Errors
            StatusCode::UnprocessableEntity => Err(Error::UnprocessableEntity(response.json()?))?,
            e => Err(Error::UnexpectedStatus(e))?,
        };

        let deserialized: V::Response = response.json()?;
        Ok(deserialized.value())
    }

    fn fetch(&self, dispatch: RequestBuilder) -> Result<Response> {
        let response = dispatch
            .header(Authorization(Bearer {
                token: self.token.clone(),
            })).send()?;
        
        info!("Response status: {:?}", response.status());
        Ok(response)
    }
}