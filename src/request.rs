pub use error::{Error, Result};
use url::Url;
use method::{Method};
use serde_json::Value;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Request<M, O> where O: ?Sized, M: Method {
    pub url: Url,
    pub body: Option<Value>,
    method: PhantomData<M>,
    response_type: PhantomData<O>,
}

impl<M, O> Request<M, O>
where M: Method {
    pub fn new(url: Url) -> Self {
        Request {
            url: url,
            body: None,
            method: PhantomData,
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