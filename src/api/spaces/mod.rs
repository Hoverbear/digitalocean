mod bucket;
mod object;

use crate::method::Method;

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub use self::bucket::Bucket;
pub use self::object::{ContentDisposition, ObjectACL};

/// Wrapper for Spaces API requests.
pub struct SpacesRequest<M, A, V> {
    /// Model used in the request.
    model: M,
    method: PhantomData<A>,
    value: PhantomData<V>,
}

impl<M, A, V> Deref for SpacesRequest<M, A, V> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        &self.model
    }
}

impl<M, A, V> DerefMut for SpacesRequest<M, A, V> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.model
    }
}

impl<M, A, V> SpacesRequest<M, A, V>
where
    A: Method,
{
    /// Creates this wrapper for the given model.
    pub(crate) fn from_model(m: M) -> Self {
        SpacesRequest {
            model: m,
            method: PhantomData,
            value: PhantomData,
        }
    }
}

/// Response returned by Spaces API whenever an error occurs.
#[derive(Debug, Deserialize)]
pub struct SpacesError {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "RequestId")]
    pub request_id: Option<String>,
    #[serde(rename = "HostId")]
    pub host_id: Option<String>,
}
