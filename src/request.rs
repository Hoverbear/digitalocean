//! Builder structure used by all requests.

pub use error::{Error, Result};
use url::Url;
use serde::Deserialize;
use serde_json::Value;
use std::marker::PhantomData;
use api::{HasValue, HasPagination, HasResponse};
use action::{Action, List, Get, Create, Delete, Update};
use DigitalOcean;

#[derive(Debug, Clone)]
pub struct Request<A, R> where A: Action {
    pub url: Url,
    pub body: Value,
    pub action: PhantomData<A>,
    pub value: PhantomData<R>,
}

impl<A, V> Request<A, V>
where A: Action {
    pub fn new(url: Url) -> Self {
        Request {
            url: url,
            body: Value::Null,
            action: PhantomData,
            value: PhantomData,
        }
    }
    pub fn body(mut self, body: Value) -> Self {
        self.body = body;
        self
    }
    pub fn url(mut self, url: Url) -> Self {
        self.url = url;
        self
    }
    pub fn action<B>(self) -> Request<B, V>
    where B: Action {
        Request::new(self.url).body(self.body)
    }
    pub fn value<B>(self) -> Request<A, B> {
        Request::new(self.url).body(self.body)
    }
}

pub trait Executable<T>: Sized
where T: Deserialize + Clone + HasResponse,
      T::Response: HasValue<Value=T> {
    fn execute(self, instance: &DigitalOcean) -> Result<T>;
}

impl<V> Executable<Vec<V>> for Request<List, Vec<V>>
where Vec<V>: HasResponse,
      V: Deserialize + Clone,
      <Vec<V> as HasResponse>::Response: HasValue<Value=Vec<V>> + HasPagination {
    fn execute(self, instance: &DigitalOcean) -> Result<Vec<V>> {
        let response: Vec<V> = instance.list(self)?;
        Ok(response)
    }
}

impl<V> Executable<V> for Request<Create, V>
where V: Deserialize + Clone + HasResponse,
      V::Response: HasValue<Value=V> {
    fn execute(self, instance: &DigitalOcean) -> Result<V> {
        let response = instance.post(self)?;
        Ok(response)
    }
}

impl<V> Executable<V> for Request<Update, V>
where V: Deserialize + Clone + HasResponse,
      V::Response: HasValue<Value=V> {
    fn execute(self, instance: &DigitalOcean) -> Result<V> {
        let response = instance.put(self)?;
        Ok(response)
    }
}

impl<V> Executable<V> for Request<Get, V>
where V: Deserialize + Clone + HasResponse,
      V::Response: HasValue<Value=V> {
    fn execute(self, instance: &DigitalOcean) -> Result<V> {
        let response = instance.get(self)?;
        Ok(response)
    }
}

impl Executable<()> for Request<Delete, ()> {
    fn execute(self, instance: &DigitalOcean) -> Result<()> {
        let response = instance.delete(self)?;
        Ok(response)
    }
}