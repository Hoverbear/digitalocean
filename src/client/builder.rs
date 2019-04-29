use crate::error::XmlError;
use failure::Error;
use hyper::{Body, Request};
use serde::Serialize;
use url::percent_encoding::{self as url_encode, DEFAULT_ENCODE_SET, QUERY_ENCODE_SET};

use std::collections::BTreeMap;
use std::fmt::Display;

/// Interface for API request builders.
pub(crate) trait RequestBuilder {
    /// Path stored in this builder.
    fn raw_path(&self) -> Option<&str>;
    /// Reference to the request payload.
    fn raw_payload(&self) -> &Option<Vec<u8>>;
    /// Mutable reference to the request payload.
    fn raw_payload_mut(&mut self) -> &mut Option<Vec<u8>>;
    /// Params used for the builder's URL.
    fn raw_params(&self) -> &BTreeMap<String, String>;
    /// Mutable reference to the underlying query params.
    fn raw_params_mut(&mut self) -> &mut BTreeMap<String, String>;
    /// Builds the `Request` object to be used in the HTTP client.
    fn build_request(self) -> Result<Request<Body>, Error>;

    /// Get a reference to the payload for this request.
    #[inline]
    fn payload(&self) -> &[u8] {
        self.raw_payload()
            .as_ref()
            .map(Vec::as_slice)
            .unwrap_or(&[] as &[u8])
    }

    /// Set the payload by serializing the given model as JSON.
    fn payload_json<T>(&mut self, model: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        let bytes = serde_json::to_vec(model)?;
        self.raw_payload_mut().replace(bytes);
        Ok(())
    }

    /// Set the payload by serializing the given model as XML.
    fn payload_xml<T>(&mut self, model: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        let mut bytes = Vec::new();
        serde_xml_rs::to_writer(&mut bytes, model).map_err(XmlError::from)?;
        self.raw_payload_mut().replace(bytes);
        Ok(())
    }

    /// Insert an URL query parameter.
    fn param<K, V>(&mut self, key: K, value: V)
    // FIXME: Maybe a better way than `Display`?
    where
        K: Display,
        V: Display,
    {
        self.raw_params_mut()
            .insert(key.to_string(), value.to_string());
    }

    /// Relative URL path used in this request.
    #[inline]
    fn path(&self) -> String {
        self.raw_path()
            .map(|s| url_encode::utf8_percent_encode(s, QUERY_ENCODE_SET).collect::<String>())
            .unwrap_or(String::from("/"))
    }

    /// Query parameters percent-encoded as a string. Note that this doesn't begin with "?",
    /// so it's up to the caller to prefix "?" if the query string is empty.
    fn query_string(&self) -> String {
        let mut string = String::new();

        for (name, value) in self.raw_params() {
            if !string.is_empty() {
                string.push('&');
            }

            string.extend(url_encode::utf8_percent_encode(name, DEFAULT_ENCODE_SET));
            string.push('=');
            string.extend(url_encode::utf8_percent_encode(value, DEFAULT_ENCODE_SET));
        }

        string
    }
}
