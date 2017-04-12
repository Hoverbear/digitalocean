pub use error::{Error, Result};
use url::Url;
use serde_json::Value;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Request<'url, Output> {
    pub method: Method,
    pub url: &'url Url,
    pub body: Option<Value>,
    response_type: PhantomData<Output>,
    pub paginated: bool,
    pub max_items: Option<usize>,
}

impl<'url, Output> Request<'url, Output> {
    pub fn new(method: Method, url: &'url Url) -> Self {
        Request {
            method: method,
            url: url,
            body: None,
            response_type: PhantomData,
            paginated: false,
            max_items: None,
        }
    }
    pub fn body<'a>(&'a mut self, body: Value) -> &'a mut Self {
        self.body = Some(body);
        self
    }
    pub fn url<'a>(&'a mut self, url: &'url Url) -> &'a mut Self {
        self.url = url;
        self
    }
    pub fn paginated<'a>(&'a mut self, setting: bool) -> &'a mut Self {
        self.paginated = setting;
        self
    }
    pub fn max_items<'a>(&'a mut self, max_items: Option<usize>) -> &'a mut Self {
        self.max_items = max_items;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Method {
    Get,
    Post,
    Delete
}