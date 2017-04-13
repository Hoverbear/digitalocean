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
mod action;
pub mod request;

use reqwest::Client;
use reqwest::header::{Authorization, Bearer};
use reqwest::StatusCode;
use reqwest::{RequestBuilder, Response};
use request::Request;
use action::{List, Get, Post, Delete};
pub use error::{Error, Result};
use url::Url;
use serde::Deserialize;
use std::iter::FromIterator;


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

    fn get<R>(&self, request: Request<Get, R>) -> Result<R>
    where R: Deserialize + Clone {
        info!("GET {:?}", request.url);
        let req = self.client.get(request.url.clone());

        let mut response = self.fetch(req)?;
        
        match *response.status() {
            // Successes
            StatusCode::Ok => (), // Get success
            // Errors
            e => Err(Error::UnexpectedStatus(e))?,
        };
        
        let deserialized: R = response.json()?;
        Ok(deserialized)
    }

    fn list<R>(&self, request: Request<List, R>) -> Result<Vec<R>>
    where R: Deserialize + Clone + HasValue {
        info!("Retrieving GET.");
        // This may be a paginated response. We need to buffer.
        let mut buffer = Vec::new();
        let mut current_url = request.url.clone();

        loop {
            let current_request = Request::new(current_url);
            let deserialized: R = self.get(current_request)?;
            let next_page = deserialized.next_page();
            buffer.push(deserialized);
            current_url = match next_page {
                Some(v) => v,
                None => break,
            };
        }

        Ok(buffer)
    }

    // Delete requests do not return content.
    fn delete<R>(&self, request: Request<Delete, R>) -> Result<()> {
        info!("DELETE {:?}", request.url);
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

    fn post<R>(&self, request: Request<Post, R>) -> Result<R>
    where R: Deserialize + Clone {
        info!("POST {:?}", request.url);
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

        let deserialized: R = response.json()?;
        Ok(deserialized)
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

pub trait Retrievable<T>: Sized {
    fn retrieve(self, instance: &DigitalOcean) -> Result<T>;
}

impl<R> Retrievable<R::Value> for Request<List, R>
where R: Deserialize + Clone + HasValue, R::Value: IntoIterator + FromIterator<<R::Value as IntoIterator>::Item> {
    fn retrieve(self, instance: &DigitalOcean) -> Result<R::Value> {
        info!("Retrieving GET list.");
        let responses = instance.list::<R>(self)?;
        let items = responses.into_iter()
            .flat_map(|v| v.value())
            .collect();
        Ok(items)
    }
}

impl<R> Retrievable<R::Value> for Request<Post, R>
where R: Deserialize + Clone + HasValue {
    fn retrieve(self, instance: &DigitalOcean) -> Result<R::Value> {
        info!("Retrieving GET.");
        let response = instance.post::<R>(self)?;
        Ok(response.value())
    }
}

impl<R> Retrievable<R::Value> for Request<Get, R>
where R: Deserialize + Clone + HasValue {
    fn retrieve(self, instance: &DigitalOcean) -> Result<R::Value> {
        info!("Retrieving GET.");
        let response = instance.get::<R>(self)?;
        Ok(response.value())
    }
}

impl Retrievable<()> for Request<Delete, ()> {
    fn retrieve(self, instance: &DigitalOcean) -> Result<()> {
        info!("Retrieving GET.");
        let response = instance.delete::<()>(self)?;
        Ok(response)
    }
}

pub trait HasValue {
    type Value: Deserialize;
    fn next_page(&self) -> Option<Url>;
    fn value(self) -> Self::Value;
}