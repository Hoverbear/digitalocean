//! Abstract types representing requests and how they are executed.
//!
//!

use error::*;
use url_serde;
use url::Url;
use serde_json::Value;
use std::marker::PhantomData;
use api::{HasPagination, HasResponse};
use method::{Method, List, Get, Create, Delete, Update};
use DigitalOcean;

/// A consuming builder which can be used to build up API calls.
///
/// In general consumers of the crate should not need to use this type directly.
/// Instead, build up requests from what is found in [`api::*`](../api/index.html).
#[derive(Debug, Clone, Serialize, Deserialize, MutGetters, Getters, Setters)]
pub struct Request<A, R>
    where A: Method
{
    #[get_mut = "pub"] #[set = "pub"] #[get = "pub"]
    #[serde(with = "url_serde")]
    url: Url,
    /// The JSON body of the request. 
    #[get_mut = "pub"] #[set = "pub"] #[get = "pub"]
    body: Value,
    #[get = "pub"]
    method: A,
    value: PhantomData<R>,
}

impl<A, V> Request<A, V>
    where A: Method
{
    /// Create a request pointing at the given url. `V` is the value ultimately
    /// returned when the call is executed.
    pub fn new(url: Url) -> Self {
        Request {
            url: url,
            body: Value::Null,
            method: A::default(),
            value: PhantomData,
        }
    }
    pub(crate) fn transmute<C, D>(self) -> Request<C, D>
    where C: Method {
        let mut req = Request::new(self.url);
        req.set_body(self.body);
        req
    }
}

impl<V> Request<List, V> {
    /// Impose a limit on the number of values which may be retrieved from a request.
    pub fn limit(mut self, limit: Option<usize>) -> Self {
        self.method.0 = limit;
        self
    }
}

/// Describes an API call which can be executed.
pub trait Executable<T>: Sized
    where T: HasResponse
{
    /// Execute the corresponding call.
    fn execute(self, instance: &DigitalOcean) -> Result<T>;
}

impl<V> Executable<Vec<V>> for Request<List, Vec<V>>
    where Vec<V>: HasResponse,
          <Vec<V> as HasResponse>::Response: HasPagination
{
    fn execute(self, instance: &DigitalOcean) -> Result<Vec<V>> {
        let response: Vec<V> = instance.list(self)?;
        Ok(response)
    }
}

impl<V> Executable<V> for Request<Create, V>
    where V: HasResponse
{
    fn execute(self, instance: &DigitalOcean) -> Result<V> {
        let response = instance.post(self)?;
        Ok(response)
    }
}

impl<V> Executable<V> for Request<Update, V>
    where V: HasResponse
{
    fn execute(self, instance: &DigitalOcean) -> Result<V> {
        let response = instance.put(self)?;
        Ok(response)
    }
}

impl<V> Executable<V> for Request<Get, V>
    where V: HasResponse
{
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
