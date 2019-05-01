use crate::request::{ApiRequest, SpacesRequestBuilder};
use crate::error::{ErrorKind, XmlError};
use failure::Error;
use futures::{future, Stream};
use http::method::Method;
use hyper::client::{Client, HttpConnector};
use hyper_rustls::HttpsConnector;
use log::Level;
use serde::Deserialize;

use std::fmt::Debug;

/// A Spaces client that holds access and secret keys.
#[derive(Clone)]
pub struct Spaces {
    client: Client<HttpsConnector<HttpConnector>>,
    access_key: String,
    secret_key: String,
}

impl Spaces {
    /// Create a Spaces client with the given access and secret keys.
    pub fn new<A, S>(access_key: A, secret_key: S) -> Self
    where
        A: Into<String>,
        S: Into<String>,
    {
        // FIXME: How many threads for connector? `num_cpus`? Or configurable?
        let connector = HttpsConnector::new(4);
        let client = Client::builder().build(connector);
        Spaces {
            access_key: access_key.into(),
            secret_key: secret_key.into(),
            client,
        }
    }

    /// Initializes the request builder with the provided HTTP method, region and bucket name.
    #[inline]
    pub(crate) fn builder(
        &self,
        method: Method,
        region: &str,
        bucket: &str,
    ) -> SpacesRequestBuilder {
        SpacesRequestBuilder::new(method, region, bucket, &self.access_key, &self.secret_key)
    }

    pub(crate) async fn fetch_response<'a, R>(&'a self, request: ApiRequest) -> Result<R, Error>
    where
        for<'de> R: Deserialize<'de>,
        R: Debug,
    {
        let msg = format!("{}: {}", request.method(), request.uri());
        let response = await!(self.client.request(request))?;
        info!("{}: {}", msg, response.status());

        let code = response.status().as_u16();
        let f = response.into_body().fold(vec![], |mut body, ref chunk| {
            body.extend_from_slice(chunk);
            future::ok::<_, hyper::Error>(body)
        });

        let bytes = await!(f)?;
        if code < 200 || code >= 300 {
            return Err(ErrorKind::Spaces(Self::parse_xml(bytes)?))?;
        }

        Ok(Self::parse_xml(bytes)?)
    }

    /// Parse the given bytes as XML and return a future that resolves to that object.
    pub(crate) fn parse_xml<T, B>(bytes: B) -> Result<T, Error>
    where
        for<'de> T: Deserialize<'de>,
        T: Debug,
        B: AsRef<[u8]>,
    {
        if log_enabled!(Level::Debug) {
            let string = String::from_utf8_lossy(bytes.as_ref());
            debug!("Actual response: {}", string);
        }

        let result: Result<T, _> = serde_xml_rs::from_reader(bytes.as_ref());
        debug!("Deserialized: {:?}", result);
        Ok(result.map_err(XmlError::from)?)
    }
}
