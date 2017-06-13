use serde::Serialize;
use std::fmt::Display;
use request::Request;
use method::{List, Get, Create, Delete};
use {ROOT_URL, STATIC_URL_ERROR};
use chrono::{DateTime, UTC};
use url::Url;
use super::{ApiLinks, ApiMeta};
use super::{HasValue, HasPagination, HasResponse};

const CERTIFICATES_SEGMENT: &'static str = "certificates";

/// SSL certificates may be uploaded to DigitalOcean where they will be placed
/// in a fully encrypted and isolated storage system. They may then be used to
/// perform SSL termination on Load Balancers.
///
/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#certificates)
#[derive(Deserialize, Serialize, Debug, Clone, Getters, Setters)]
pub struct Certificate {
    /// A unique ID that can be used to identify and reference a certificate.
    #[get = "pub"]
    id: String,
    /// A unique human-readable name referring to a certificate.
    #[get = "pub"]
    name: String,
    /// A time value given in ISO8601 combined date and time format that
    /// represents the certificate's expiration date.
    #[get = "pub"]
    not_after: DateTime<UTC>,
    /// A unique identifier generated from the SHA-1 fingerprint of the
    /// certificate.
    #[get = "pub"]
    sha1_fingerprint: String,
    /// A time value given in ISO8601 combined date and time format that
    /// represents when the certificate was created.
    #[get = "pub"]
    created_at: DateTime<UTC>,
}

impl Certificate {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-certificate)
    pub fn create<S>(name: S, private_key: S, leaf_certificate: S) -> Request<Create, Certificate>
        where S: AsRef<str> + Serialize + Display
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(CERTIFICATES_SEGMENT);

        let mut req = Request::new(url);
        
        req.set_body(json!({
            "name": name,
            "private_key": private_key,
            "leaf_certificate": leaf_certificate,
        }));

        req
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-certificates)
    pub fn list() -> Request<List, Vec<Certificate>> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(CERTIFICATES_SEGMENT);

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-certificate)
    pub fn get<N>(id: N) -> Request<Get, Certificate>
        where N: AsRef<str> + Display
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(CERTIFICATES_SEGMENT)
            .push(id.as_ref());

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#delete-a-certificate)
    pub fn delete<N>(id: N) -> Request<Delete, ()>
        where N: AsRef<str> + Display
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(CERTIFICATES_SEGMENT)
            .push(id.as_ref());

        Request::new(url)
    }
}

impl Request<Create, Certificate> {
    /// The full PEM-formatted trust chain between the certificate authority's
    /// certificate and your domain's SSL certificate.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-certificate)
    pub fn certificate_chain<S>(mut self, val: S) -> Self
        where S: AsRef<str> + Serialize + Display
    {
        self.body_mut()["certificate_chain"] = json!(val);
        self
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CertificateResponse {
    certificate: Certificate,
}

impl HasResponse for Certificate {
    type Response = CertificateResponse;
}

impl HasValue for CertificateResponse {
    type Value = Certificate;
    fn value(self) -> Certificate {
        self.certificate
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CertificateListResponse {
    certificates: Vec<Certificate>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasResponse for Vec<Certificate> {
    type Response = CertificateListResponse;
}

impl HasPagination for CertificateListResponse {
    fn next_page(&self) -> Option<Url> {
        self.links.next()
    }
}

impl HasValue for CertificateListResponse {
    type Value = Vec<Certificate>;
    fn value(self) -> Vec<Certificate> {
        self.certificates
    }
}
