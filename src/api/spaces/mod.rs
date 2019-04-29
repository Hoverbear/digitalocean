mod bucket;
mod object;

use crate::method::Method;

use std::marker::PhantomData;

pub use self::bucket::Bucket;

/// Wrapper for Spaces API requests.
pub struct SpacesRequest<M, A, V> {
    /// Model used in the request.
    pub(crate) model: M,
    method: PhantomData<A>,
    value: PhantomData<V>,
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
