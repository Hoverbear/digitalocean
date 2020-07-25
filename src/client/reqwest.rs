pub use reqwest::Client;

use crate::api::{HasPagination, HasResponse, HasValue, MAX_PER_PAGE};
use crate::error::Error;
use crate::method::{Create, Delete, Get, List, Update};
use crate::request::Request;
use crate::DigitalOcean;
use log::info;
use reqwest::StatusCode;
use reqwest::{RequestBuilder, Response};

impl DigitalOcean {
    pub(crate) fn get<V>(&self, request: Request<Get, V>) -> Result<V, Error>
    where
        V: HasResponse,
    {
        info!("GET {:?}", request.url());
        let req = self.client.get(request.url().clone());

        let mut response = self.fetch(req)?;

        match response.status() {
            // Successes
            StatusCode::OK => (),
            // Not Found
            StatusCode::NOT_FOUND => Err(Error::NotFound)?,
            // Errors
            e => Err(Error::UnexpectedStatus(e))?,
        };

        let deserialized: V::Response = response.json()?;
        Ok(deserialized.value())
    }

    pub(crate) fn list<V>(&self, request: Request<List, Vec<V>>) -> Result<Vec<V>, Error>
    where
        Vec<V>: HasResponse,
        <Vec<V> as HasResponse>::Response: HasPagination,
    {
        info!("LIST {:?}", request.url());
        // This may be a paginated response. We need to buffer.
        let mut buffer = Vec::new();
        let mut current_url = request.url().clone();

        match request.method().0 {
            Some(limit) if limit < MAX_PER_PAGE => {
                current_url
                    .query_pairs_mut()
                    .append_pair("per_page", &limit.to_string());
            }
            _ => {
                current_url
                    .query_pairs_mut()
                    .append_pair("per_page", &MAX_PER_PAGE.to_string());
            }
        };

        loop {
            let req = self.client.get(current_url.clone());
            let mut response = self.fetch(req)?;

            match response.status() {
                StatusCode::OK => (),
                // Not Found
                StatusCode::NOT_FOUND => Err(Error::NotFound)?,
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

            if let Some(limit) = request.method().0 {
                let buffer_size = buffer.len();
                let remaining = limit - buffer_size;
                if buffer_size >= limit {
                    break;
                } else if remaining < MAX_PER_PAGE {
                    current_url
                        .query_pairs_mut()
                        .append_pair("per_page", &remaining.to_string());
                }
            }
            info!("Fetching next page...")
        }

        Ok(buffer)
    }

    // Delete requests do not return content.
    pub(crate) fn delete<V>(&self, request: Request<Delete, V>) -> Result<(), Error> {
        info!("DELETE {:?}", request.url());
        let req = self.client.delete(request.url().clone());

        let response = self.fetch(req)?;

        match response.status() {
            // Successes
            StatusCode::NO_CONTENT => (), // Delete success
            // Errors
            e => Err(Error::UnexpectedStatus(e))?,
        };

        Ok(())
    }

    pub(crate) fn post<V>(&self, request: Request<Create, V>) -> Result<V, Error>
    where
        V: HasResponse,
    {
        info!("POST {:?}", request.url());
        let req = self
            .client
            .post(request.url().clone())
            .json(&request.body().clone());

        let mut response = self.fetch(req)?;

        match response.status() {
            // Successes
            StatusCode::CREATED => (),  // Post Success
            StatusCode::ACCEPTED => (), // Post Success (async)
            // Errors
            StatusCode::UNPROCESSABLE_ENTITY => {
                Err(Error::UnprocessableEntity(response.json()?))?
            }
            e => Err(Error::UnexpectedStatus(e))?,
        };

        let deserialized: V::Response = response.json()?;
        Ok(deserialized.value())
    }

    pub(crate) fn put<V>(&self, request: Request<Update, V>) -> Result<V, Error>
    where
        V: HasResponse,
    {
        info!("PUT {:?}", request.url());
        let req = self
            .client
            .put(request.url().clone())
            .json(&request.body().clone());

        let mut response = self.fetch(req)?;

        match response.status() {
            // Successes
            StatusCode::OK => (), // Update success
            // Errors
            StatusCode::UNPROCESSABLE_ENTITY => {
                Err(Error::UnprocessableEntity(response.json()?))?
            }
            e => Err(Error::UnexpectedStatus(e))?,
        };

        let deserialized: V::Response = response.json()?;
        Ok(deserialized.value())
    }

    fn fetch(&self, dispatch: RequestBuilder) -> Result<Response, Error> {
        let response = dispatch.bearer_auth(self.token.clone()).send()?;

        info!("Response status: {:?}", response.status());
        Ok(response)
    }
}
