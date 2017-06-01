pub use reqwest::Client;

use reqwest::header::{Authorization, Bearer};
use reqwest::StatusCode;
use reqwest::{RequestBuilder, Response};
use DigitalOcean;
use error::*;
use request::Request;
use method::{Get, Update, Delete, Create, List};
use api::{HasPagination, HasResponse, HasValue, MAX_PER_PAGE};

impl DigitalOcean {
    pub fn get<V>(&self, request: Request<Get, V>) -> Result<V>
        where V: HasResponse
    {
        info!("GET {:?}", request.url);
        let req = self.client.get(request.url.clone());

        let mut response = self.fetch(req)?;

        match *response.status() {
            // Successes
            StatusCode::Ok => (),
            // Not Found
            StatusCode::NotFound => Err(ErrorKind::NotFound)?,
            // Errors
            e => Err(ErrorKind::UnexpectedStatus(e))?,
        };

        let deserialized: V::Response = response.json()?;
        Ok(deserialized.value())
    }

    pub fn list<V>(&self, request: Request<List, Vec<V>>) -> Result<Vec<V>>
        where Vec<V>: HasResponse,
              <Vec<V> as HasResponse>::Response: HasPagination
    {
        info!("LIST {:?}", request.url);
        // This may be a paginated response. We need to buffer.
        let mut buffer = Vec::new();
        let mut current_url = request.url.clone();

        current_url
            .query_pairs_mut()
            .append_pair("per_page", &MAX_PER_PAGE.to_string());

        loop {
            let req = self.client.get(current_url.clone());
            let mut response = self.fetch(req)?;

            match *response.status() {
                // Successes
                StatusCode::Ok => (),
                // Not Found
                StatusCode::NotFound => Err(ErrorKind::NotFound)?,
                // Errors
                e => Err(ErrorKind::UnexpectedStatus(e))?,
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
    pub fn delete<V>(&self, request: Request<Delete, V>) -> Result<()> {
        info!("DELETE {:?}", request.url);
        let req = self.client.delete(request.url.clone());

        let response = self.fetch(req)?;

        match *response.status() {
            // Successes
            StatusCode::NoContent => (), // Delete success
            // Errors
            e => Err(ErrorKind::UnexpectedStatus(e))?,
        };

        Ok(())
    }

    pub fn post<V>(&self, request: Request<Create, V>) -> Result<V>
        where V: HasResponse
    {
        info!("POST {:?}", request.url);
        let req = self.client.post(request.url.clone());

        let req = req.json(&request.body.clone());

        let mut response = self.fetch(req)?;

        match *response.status() {
            // Successes
            StatusCode::Created => (), // Post Success
            // Errors
            StatusCode::UnprocessableEntity => {
                Err(ErrorKind::UnprocessableEntity(response.json()?))?
            }
            e => Err(ErrorKind::UnexpectedStatus(e))?,
        };

        let deserialized: V::Response = response.json()?;
        Ok(deserialized.value())
    }

    pub fn put<V>(&self, request: Request<Update, V>) -> Result<V>
        where V: HasResponse
    {
        info!("PUT {:?}", request.url);
        let req = self.client.put(request.url.clone());

        let req = req.json(&request.body.clone());

        let mut response = self.fetch(req)?;

        match *response.status() {
            // Successes
            StatusCode::Ok => (), // Update success
            // Errors
            StatusCode::UnprocessableEntity => {
                Err(ErrorKind::UnprocessableEntity(response.json()?))?
            }
            e => Err(ErrorKind::UnexpectedStatus(e))?,
        };

        let deserialized: V::Response = response.json()?;
        Ok(deserialized.value())
    }

    fn fetch(&self, dispatch: RequestBuilder) -> Result<Response> {
        let response = dispatch
            .header(Authorization(Bearer { token: self.token.clone() }))
            .send()?;

        info!("Response status: {:?}", response.status());
        Ok(response)
    }
}
