use reqwest;
use error::Result;
use reqwest::{RequestBuilder, IntoUrl};
use reqwest::header::{Authorization, Bearer};

#[derive(Clone, Debug)]
pub struct Client {
    client: reqwest::Client,
    token: String,
}

impl Client {
    pub fn new<T: Into<String>>(token: T) -> Result<Self> {
        Ok(Client {
            client: reqwest::Client::new()?,
            token: token.into(),
        })
    }

    // Make an authenticated GET request.
    pub fn get<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.client
            .get(url)
            .header(Authorization(Bearer {
                token: self.token.clone(),
            }))
    }

    pub fn post<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.client
            .post(url)
            .header(Authorization(Bearer {
                token: self.token.clone(),
            }))
    }

    pub fn delete<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.client
            .delete(url)
            .header(Authorization(Bearer {
                token: self.token.clone(),
            }))
    }
}
