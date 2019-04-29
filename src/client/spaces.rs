use super::RequestBuilder;
use crate::error::{ErrorKind, XmlError};
use chrono::offset::Utc;
use chrono::DateTime;
use failure::Error;
use futures::{future, Stream};
use hmac::{Hmac, Mac};
use http::header::{self, HeaderMap, HeaderName, HeaderValue};
use http::method::Method;
use hyper::client::{Client, HttpConnector};
use hyper::{Body, Request};
use hyper_rustls::HttpsConnector;
use log::Level;
use serde::Deserialize;
use sha2::{Digest as Sha256Digest, Sha256};

use std::collections::BTreeMap;
use std::fmt::Debug;

type HmacSha256 = Hmac<Sha256>;

lazy_static! {
    /// Access control list header for resources.
    pub(crate) static ref ACL_HEADER: HeaderName = HeaderName::from_static("x-amz-acl");
    /// Date header used in authorization.
    pub(crate) static ref DATE_HEADER: HeaderName = HeaderName::from_static("x-amz-date");
    /// Hash header used in authorization.
    pub(crate) static ref HASH_HEADER: HeaderName = HeaderName::from_static("x-amz-content-sha256");
    /// Headers that are ignored in spaces request.
    static ref SPACES_IGNORED_HEADERS: [HeaderName; 4] = [
        header::AUTHORIZATION, header::CONTENT_LENGTH, header::CONTENT_TYPE, ACL_HEADER.clone()
    ];
}

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

    pub(crate) async fn fetch_response<'a>(
        &'a self,
        builder: SpacesRequestBuilder,
    ) -> Result<Vec<u8>, Error> {
        let request = builder.build_request()?;
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

        Ok(bytes)
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

/// HTTP request builder for Spaces API.
// NOTE: This struct is accessible only within the crate, so we don't have to
// worry about the fields and methods being public.
// FIXME: A number of owned strings should be borrows here.
pub(crate) struct SpacesRequestBuilder {
    /// Relative path of request URL.
    pub path: Option<String>,
    /// Payload for this request.
    pub payload: Option<Vec<u8>>,
    /// Headers used in this request.
    pub headers: HeaderMap,
    /// Content type of this request.
    pub content_type: Option<HeaderValue>,
    /// `User-Agent` header to be set in the request.
    pub user_agent: Option<String>,

    method: Method,
    region: String,
    params: BTreeMap<String, String>,
    host: String,
    access_key: String,
    secret_key: String,
}

impl SpacesRequestBuilder {
    /// Spaces hostname.
    const SPACES_HOST: &'static str = "digitaloceanspaces.com";
    /// Default user agent (set in request header).
    const DEFAULT_USER_AGENT: &'static str = "ocean";

    /// Creates a new builder using the necessary parameters.
    pub fn new(
        method: Method,
        region: &str,
        bucket: &str,
        access_key: &str,
        secret_key: &str,
    ) -> Self {
        SpacesRequestBuilder {
            method,
            region: String::from(region),
            path: None,
            host: format!("{}.{}.{}", bucket, region, Self::SPACES_HOST),
            payload: None,
            headers: HeaderMap::with_capacity(5),
            user_agent: None,
            content_type: None,
            params: BTreeMap::new(),
            access_key: String::from(access_key),
            secret_key: String::from(secret_key),
        }
    }

    fn sign(&mut self) -> Result<(), Error> {
        // Set `Host` header.
        let host = HeaderValue::from_str(&self.host).map_err(|_| ErrorKind::InvalidHostHeader)?;
        self.headers.insert(header::HOST, host);

        // Set `User-Agent` header.
        let agent = HeaderValue::from_str(
            self.user_agent
                .as_ref()
                .map(String::as_str)
                .unwrap_or(Self::DEFAULT_USER_AGENT),
        )
        .map_err(|_| ErrorKind::InvalidUserAgent)?;
        self.headers.insert(header::USER_AGENT, agent);

        // Set the date used in the signature.
        let date = Utc::now();
        let value = date.format("%Y%m%dT%H%M%SZ").to_string();
        self.headers.insert(
            DATE_HEADER.clone(),
            HeaderValue::from_str(&value).expect("datetime should've been valid"),
        );

        // Gather necessary components for signing.
        let signed_headers = self.signed_headers();
        let canonical_uri = self.path();
        let canonical_headers = self.canonical_headers();
        let canonical_query_string = self.query_string();

        // Get the hash of payload and set content length.
        let hash = match self.payload {
            Some(ref bytes) => {
                self.headers.insert(
                    header::CONTENT_LENGTH,
                    HeaderValue::from_str(&bytes.len().to_string())
                        .expect("length should've been valid"),
                );
                Self::sha2_lower_hex(bytes)
            }
            None => Self::sha2_lower_hex(b""),
        };

        // Set the hash for payload.
        self.headers.insert(
            HASH_HEADER.clone(),
            HeaderValue::from_str(&hash).expect("hash digest should've been valid"),
        );

        // Build the canonical request.
        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n\n{}\n{}",
            self.method.as_str(),
            canonical_uri,
            canonical_query_string,
            canonical_headers,
            signed_headers,
            &hash
        );

        // Set `Content-Type` header.
        let content_type = self
            .content_type
            .clone()
            .unwrap_or(HeaderValue::from_static("application/octet-stream"));
        self.headers.insert(header::CONTENT_TYPE, content_type);

        // Hash the request and sign appropriately.
        let hashed_request = Self::sha2_lower_hex(canonical_request.as_bytes());
        let scope = format!(
            "{}/{}/s3/aws4_request",
            date.format("%Y%m%d").to_string(),
            &self.region
        );
        let sign_message = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            date.format("%Y%m%dT%H%M%SZ"),
            scope,
            hashed_request
        );
        let sign = self.signature(date, &sign_message);

        // Set the `Authorization` header using the signature.
        let auth = format!(
            "AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
            &self.access_key, scope, signed_headers, sign
        );
        self.headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&auth).expect("expected valid auth header"),
        );

        Ok(())
    }

    /// Header names used in signing the request.
    fn signed_headers(&self) -> String {
        let mut joined = String::new();
        self.headers.keys().for_each(|name| {
            if SPACES_IGNORED_HEADERS.contains(&name) {
                return;
            }

            if !joined.is_empty() {
                joined.push(';');
            }

            joined.push_str(name.as_str());
        });

        joined
    }

    /// Headers (names and values) used in signing (joined by newline).
    fn canonical_headers(&self) -> String {
        let mut joined = String::new();
        self.headers.iter().for_each(|(name, value)| {
            if SPACES_IGNORED_HEADERS.contains(&name) {
                return;
            }

            if !joined.is_empty() {
                joined.push('\n');
            }

            joined.push_str(&format!(
                "{}:{}",
                name,
                value.to_str().expect("expected valid header values")
            ))
        });

        joined
    }

    /// Generates the HMAC-256 signature required by the Spaces API based on the date and message.
    fn signature(&self, date: DateTime<Utc>, message: &str) -> String {
        let mut mac = HmacSha256::new_varkey(format!("AWS4{}", self.secret_key).as_bytes())
            .expect("any key size is allowed?");
        let mut final_mac = Default::default();

        let date = date.format("%Y%m%d").to_string();
        let mac_input = &[
            date.as_bytes(),
            self.region.as_bytes(),
            b"s3",
            b"aws4_request",
            message.as_bytes(),
        ];

        for (i, input) in mac_input.into_iter().enumerate() {
            mac.input(input);
            final_mac = mac.result().code();
            if i == mac_input.len() - 1 {
                break; // Avoid unnecessary MAC computation in final iteration.
            }

            mac = HmacSha256::new_varkey(&final_mac).expect("any key size is allowed?");
        }

        format!("{:x}", final_mac)
    }

    /// Generates the SHA-256 hash of the given byte slice and returns the lower hex format.
    #[inline]
    fn sha2_lower_hex(input: &[u8]) -> String {
        format!("{:x}", Sha256::digest(input))
    }
}

impl RequestBuilder for SpacesRequestBuilder {
    #[inline]
    fn raw_payload_mut(&mut self) -> &mut Option<Vec<u8>> {
        &mut self.payload
    }

    #[inline]
    fn raw_payload(&self) -> &Option<Vec<u8>> {
        &self.payload
    }

    #[inline]
    fn raw_path(&self) -> Option<&str> {
        self.path.as_ref().map(String::as_str)
    }

    #[inline]
    fn raw_params(&self) -> &BTreeMap<String, String> {
        &self.params
    }

    #[inline]
    fn raw_params_mut(&mut self) -> &mut BTreeMap<String, String> {
        &mut self.params
    }

    /// Signs this request, consumes the builder, and returns a `Request` object to be used
    /// in the HTTP client.
    fn build_request(mut self) -> Result<Request<Body>, Error> {
        self.sign()?;
        let mut query = self.query_string();
        if !query.is_empty() {
            query = String::from("?") + &query;
        }

        let mut request = Request::builder()
            .method(&self.method)
            .uri(format!("https://{}{}{}", &self.host, self.path(), query))
            .body(self.payload.take().map(Body::from).unwrap_or(Body::empty()))
            .expect("expected request building to succeed");

        *request.headers_mut() = self.headers;
        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use chrono::offset::Utc;
    use chrono::TimeZone;
    use http::header;
    use http::method::Method;

    use super::{RequestBuilder, SpacesRequestBuilder};
    use super::{DATE_HEADER, HASH_HEADER};

    // Test builder struct fields after initialization.
    #[test]
    fn test_builder_init() {
        let builder =
            SpacesRequestBuilder::new(Method::PUT, "nyc3", "foobar", "booya", "my_secret");

        assert_eq!(builder.method, Method::PUT);
        assert_eq!(builder.region, "nyc3");
        assert!(builder.path.is_none());
        assert_eq!(builder.path(), "/");
        assert!(builder.payload.is_none());
        assert!(builder.content_type.is_none());
        assert!(builder.params.is_empty());
        assert!(builder.headers.is_empty());
        assert_eq!(builder.host, "foobar.nyc3.digitaloceanspaces.com");
        assert_eq!(builder.access_key, "booya");
        assert_eq!(builder.secret_key, "my_secret");
    }

    // Test the authorization value generated by the request builder.
    #[test]
    fn test_auth_value() {
        let mut builder =
            SpacesRequestBuilder::new(Method::PUT, "nyc3", "foobar", "booya", "my_secret");
        builder.payload = Some(vec![97, 98, 99]); // "abc"
        builder.params.insert(String::from("acl"), String::new());
        builder.sign().unwrap();

        assert_eq!(
            builder
                .headers
                .get(&header::HOST)
                .unwrap()
                .to_str()
                .unwrap(),
            builder.host
        );
        let date_value = builder
            .headers
            .get(&*DATE_HEADER)
            .unwrap()
            .to_str()
            .unwrap();
        let sign_date = Utc.datetime_from_str(date_value, "%Y%m%dT%H%M%SZ").unwrap();
        assert_eq!(
            builder
                .headers
                .get(&header::CONTENT_LENGTH)
                .unwrap()
                .to_str()
                .unwrap(),
            "3"
        );
        let hash = builder
            .headers
            .get(&*HASH_HEADER)
            .unwrap()
            .to_str()
            .unwrap();
        assert_eq!(
            hash,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            builder
                .headers
                .get(&header::USER_AGENT)
                .unwrap()
                .to_str()
                .unwrap(),
            "ocean"
        );
        assert_eq!(
            builder
                .headers
                .get(&header::CONTENT_TYPE)
                .unwrap()
                .to_str()
                .unwrap(),
            "application/octet-stream"
        );

        let canonical_request = format!("PUT\n/\nacl=\nhost:{}\nuser-agent:ocean\nx-amz-date:{}\n\nhost;user-agent;x-amz-date\n{}",
                                        builder.host, date_value, hash);
        let hashed_request = SpacesRequestBuilder::sha2_lower_hex(canonical_request.as_bytes());
        let scope = format!(
            "{}/nyc3/s3/aws4_request",
            sign_date.format("%Y%m%d").to_string()
        );
        let message = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            date_value, scope, hashed_request
        );
        let sign = builder.signature(sign_date, &message);

        let auth = format!("AWS4-HMAC-SHA256 Credential=booya/{}, SignedHeaders=host;user-agent;x-amz-date, Signature={}", scope, sign);
        let auth_value = builder
            .headers
            .get(&header::AUTHORIZATION)
            .unwrap()
            .to_str()
            .unwrap();
        assert_eq!(auth_value, auth);
    }

    // Test the signature generated by the request builder.
    #[test]
    fn test_signature() {
        let builder =
            SpacesRequestBuilder::new(Method::PUT, "nyc3", "foobar", "booya", "my_secret");
        let date = Utc
            .datetime_from_str("20190424T000000Z", "%Y%m%dT%H%M%SZ")
            .unwrap();
        let sign = builder.signature(date, "Hello, world!");
        assert_eq!(
            sign,
            "61cb7089edf73bae33256858ed46deadc3c36aedb785f7f1551084af63aa171a"
        );
    }
}
