pub use error::{Error, Result};
use url::Url;
use serde::Deserialize;
use serde_json::Value;
use std::marker::PhantomData;
use api::{HasValue, HasPagination};
use values::HasResponse;
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
    pub fn body<'a>(&'a mut self, body: Value) -> &'a mut Self {
        self.body = body;
        self
    }
    pub fn url<'a>(&'a mut self, url: Url) -> &'a mut Self {
        self.url = url;
        self
    }
    pub fn action<'a, B>(&'a mut self) -> &'a mut Request<B, V>
    where B: Action {
        unsafe {
            &mut *(self as *mut _ as *mut Request<B, V>)
        }
    }
    pub fn value<'a, B>(&'a mut self) -> &'a mut Request<A, B> {
        unsafe {
            &mut *(self as *mut _ as *mut Request<A, B>)
        }
    }
}

pub trait Retrievable<T>: Sized
where T: Deserialize + Clone + HasResponse,
      T::Response: HasValue<Value=T> {
    fn retrieve(&mut self, instance: &DigitalOcean) -> Result<T>;
}

impl<V> Retrievable<Vec<V>> for Request<List, Vec<V>>
where Vec<V>: HasResponse,
      V: Deserialize + Clone,
      <Vec<V> as HasResponse>::Response: HasValue<Value=Vec<V>> + HasPagination {
    fn retrieve(&mut self, instance: &DigitalOcean) -> Result<Vec<V>> {
        info!("Retrieving GET list.");
        let response: Vec<V> = instance.list(self)?;
        Ok(response)
    }
}

impl<V> Retrievable<V> for Request<Create, V>
where V: Deserialize + Clone + HasResponse,
      V::Response: HasValue<Value=V> {
    fn retrieve(&mut self, instance: &DigitalOcean) -> Result<V> {
        info!("Retrieving POST.");
        let response = instance.post(self)?;
        Ok(response)
    }
}

impl<V> Retrievable<V> for Request<Update, V>
where V: Deserialize + Clone + HasResponse,
      V::Response: HasValue<Value=V> {
    fn retrieve(&mut self, instance: &DigitalOcean) -> Result<V> {
        info!("Retrieving PUT.");
        let response = instance.put(self)?;
        Ok(response)
    }
}

impl<V> Retrievable<V> for Request<Get, V>
where V: Deserialize + Clone + HasResponse,
      V::Response: HasValue<Value=V> {
    fn retrieve(&mut self, instance: &DigitalOcean) -> Result<V> {
        info!("Retrieving GET.");
        let response = instance.get(self)?;
        Ok(response)
    }
}

impl Retrievable<()> for Request<Delete, ()> {
    fn retrieve(&mut self, instance: &DigitalOcean) -> Result<()> {
        info!("Retrieving GET.");
        let response = instance.delete(self)?;
        Ok(response)
    }
}