//! Abstract types representing requests and how they are executed.
//!
//!

use crate::api::{HasPagination, HasResponse};
use crate::method::{Create, Delete, Get, List, Method, Update};
use crate::DigitalOcean;
use failure::Error;
use serde_json::Value;
use std::marker::PhantomData;
use url::Url;
use url_serde;

/// A type alias with [`Request<_, Account>`](struct.Request.html) specific functions.
pub type AccountRequest<M, V> = Request<M, V>;
/// A type alias with [`Request<_, Action>`](struct.Request.html) specific functions.
pub type ActionRequest<M, V> = Request<M, V>;
/// A type alias with [`Request<_, Certificate>`](struct.Request.html) specific functions.
pub type CertificateRequest<M, V> = Request<M, V>;
/// A type alias with [`Request<_, DomainRecord>`](struct.Request.html) specific functions.
pub type DomainRecordRequest<M, V> = Request<M, V>;
/// A type alias with [`Request<_, Domain>`](struct.Request.html) specific functions.
pub type DomainRequest<M, V> = Request<M, V>;
/// A type alias with [`Request<_, DropletAction>`](struct.Request.html) specific functions.
pub type DropletActionRequest<M, V> = Request<M, V>;
/// A type alias with [`Request<_, Droplet>`](struct.Request.html) specific functions.
pub type DropletRequest<M, V> = Request<M, V>;
/// A type alias with [`Request<_, FloatingIpAction>`](struct.Request.html) specific functions.
pub type FloatingIpActionRequest<M, V> = Request<M, V>;
/// A type alias with [`Request<_, FloatingIp>`](struct.Request.html) specific functions.
pub type FloatingIpRequest<M, V> = Request<M, V>;
/// A type alias with [`Request<_, ImageAction>`](struct.Request.html) specific functions.
pub type ImageActionRequest<M, V> = Request<M, V>;
/// A type alias with [`Request<_, Image>`](struct.Request.html) specific functions.
pub type ImageRequest<M, V> = Request<M, V>;
/// A type alias with [`Request<_, CustomImage>`](struct.Request.html) specific functions.
pub type CustomImageRequest<M, V> = Request<M, V>;
/// A type alias with [`Request<_, LoadBalancer>`](struct.Request.html) specific functions.
pub type LoadBalancerRequest<M, V> = Request<M, V>;
/// A type alias with [`Request<_, Region>`](struct.Request.html) specific functions.
pub type RegionRequest<M, V> = Request<M, V>;
/// A type alias with [`Request<_, Size>`](struct.Request.html) specific functions.
pub type SizeRequest<M, V> = Request<M, V>;
/// A type alias with [`Request<_, Snapshot>`](struct.Request.html) specific functions.
pub type SnapshotRequest<M, V> = Request<M, V>;
/// A type alias with [`Request<_, SshKey>`](struct.Request.html) specific functions.
pub type SshKeyRequest<M, V> = Request<M, V>;
/// A type alias with [`Request<_, Tag>`](struct.Request.html) specific functions.
pub type TagRequest<M, V> = Request<M, V>;
/// A type alias with [`Request<_, VolumeAction>`](struct.Request.html) specific functions.
pub type VolumeActionRequest<M, V> = Request<M, V>;
/// A type alias with [`Request<_, Volume>`](struct.Request.html) specific functions.
pub type VolumeRequest<M, V> = Request<M, V>;

/// A consuming builder which can be used to build up API calls.
///
/// In general consumers of the crate should not need to use this type directly.
/// Instead, build up requests from what is found in [`api::*`](../api/index.html).
#[derive(Debug, Clone, Serialize, Deserialize, MutGetters, Getters, Setters)]
pub struct Request<A, R>
where
    A: Method,
{
    #[get_mut = "pub"]
    #[set = "pub"]
    #[get = "pub"]
    #[serde(with = "url_serde")]
    url: Url,
    /// The JSON body of the request.
    #[get_mut = "pub"]
    #[set = "pub"]
    #[get = "pub"]
    body: Value,
    #[get = "pub"]
    method: A,
    value: PhantomData<R>,
}

impl<A, V> Request<A, V>
where
    A: Method,
{
    /// Create a request pointing at the given url. `V` is the value ultimately
    /// returned when the call is executed.
    pub fn new(url: Url) -> Self {
        Request {
            url,
            body: Value::Null,
            method: A::default(),
            value: PhantomData,
        }
    }
    pub(crate) fn transmute<C, D>(self) -> Request<C, D>
    where
        C: Method,
    {
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
where
    T: HasResponse,
{
    /// Execute the corresponding call.
    fn execute(self, instance: &DigitalOcean) -> Result<T, Error>;
}

impl<V> Executable<Vec<V>> for Request<List, Vec<V>>
where
    Vec<V>: HasResponse,
    <Vec<V> as HasResponse>::Response: HasPagination,
{
    fn execute(self, instance: &DigitalOcean) -> Result<Vec<V>, Error> {
        let response: Vec<V> = instance.list(self)?;
        Ok(response)
    }
}

impl<V> Executable<V> for Request<Create, V>
where
    V: HasResponse,
{
    fn execute(self, instance: &DigitalOcean) -> Result<V, Error> {
        let response = instance.post(self)?;
        Ok(response)
    }
}

impl<V> Executable<V> for Request<Update, V>
where
    V: HasResponse,
{
    fn execute(self, instance: &DigitalOcean) -> Result<V, Error> {
        let response = instance.put(self)?;
        Ok(response)
    }
}

impl<V> Executable<V> for Request<Get, V>
where
    V: HasResponse,
{
    fn execute(self, instance: &DigitalOcean) -> Result<V, Error> {
        let response = instance.get(self)?;
        Ok(response)
    }
}

impl Executable<()> for Request<Delete, ()> {
    fn execute(self, instance: &DigitalOcean) -> Result<(), Error> {
        instance.delete(self)
    }
}
