use super::{ApiLinks, ApiMeta};
use super::{HasPagination, HasResponse, HasValue};
use crate::method::{Create, Delete, Get, List};
use crate::request::DomainRequest;
use crate::request::Request;
use crate::{ROOT_URL, STATIC_URL_ERROR};
use getset::{Getters, Setters};
use serde::Serialize;
use std::fmt::Display;
use std::net::IpAddr;
use url::Url;

const DOMAINS_SEGMENT: &str = "domains";

/// Domain resources are domain names that you have purchased from a domain
/// name registrar that you are managing through the DigitalOcean DNS interface.
///
/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domains)
#[derive(Deserialize, Serialize, Debug, Clone, Getters, Setters)]
pub struct Domain {
    /// The name of the domain itself. This should follow the standard domain
    /// format of domain.TLD. For instance, example.com is a valid domain name.
    #[get = "pub"]
    name: String,
    /// This value is the time to live for the records on this domain, in
    /// seconds. This defines the time frame that clients can cache queried
    /// information before a refresh should be requested.
    #[get = "pub"]
    ttl: Option<usize>,
    /// This attribute contains the complete contents of the zone file for the
    /// selected domain. Individual domain record resources should be used to
    /// get more granular control over records. However, this attribute can
    /// also be used to get information about the SOA record, which is created
    /// automatically and is not accessible as an individual record resource.
    #[get = "pub"]
    zone_file: Option<String>,
}

impl Domain {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-domain)
    pub fn create<N, I>(name: N, ip_address: I) -> DomainRequest<Create, Domain>
    where
        N: AsRef<str> + Serialize + Display,
        I: Into<IpAddr> + Serialize + Display,
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DOMAINS_SEGMENT);

        let mut req = Request::new(url);
        req.set_body(json!({
            "name": name,
            "ip_address": ip_address,
        }));
        req
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-domains)
    pub fn list() -> DomainRequest<List, Vec<Domain>> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DOMAINS_SEGMENT);

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-domain)
    pub fn get<N>(name: N) -> DomainRequest<Get, Domain>
    where
        N: AsRef<str> + Display,
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DOMAINS_SEGMENT)
            .push(name.as_ref());

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#delete-a-domain)
    pub fn delete<N>(name: N) -> DomainRequest<Delete, ()>
    where
        N: AsRef<str> + Display,
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DOMAINS_SEGMENT)
            .push(name.as_ref());

        Request::new(url)
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DomainResponse {
    domain: Domain,
}

impl HasResponse for Domain {
    type Response = DomainResponse;
}

impl HasValue for DomainResponse {
    type Value = Domain;
    fn value(self) -> Domain {
        self.domain
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DomainListResponse {
    domains: Vec<Domain>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasResponse for Vec<Domain> {
    type Response = DomainListResponse;
}

impl HasPagination for DomainListResponse {
    fn next_page(&self) -> Option<Url> {
        self.links.next()
    }
}

impl HasValue for DomainListResponse {
    type Value = Vec<Domain>;
    fn value(self) -> Vec<Domain> {
        self.domains
    }
}
