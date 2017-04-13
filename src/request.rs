pub use error::{Error, Result};
use url::Url;
use action::{Action};
use serde_json::Value;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Request<A, R> where A: Action {
    pub url: Url,
    pub body: Option<Value>,
    pub action: PhantomData<A>,
    pub response_type: PhantomData<R>,
}

impl<A, R> Request<A, R>
where A: Action {
    pub fn new(url: Url) -> Self {
        Request {
            url: url,
            body: None,
            action: PhantomData,
            response_type: PhantomData,
        }
    }
    pub fn body<'a>(&'a mut self, body: Value) -> &'a mut Self {
        self.body = Some(body);
        self
    }
    pub fn url<'a>(&'a mut self, url: Url) -> &'a mut Self {
        self.url = url;
        self
    }
}