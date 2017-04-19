use serde::Serialize;
use std::fmt::Display;
use std::net::IpAddr;
use request::Request;
use action::{List, Get, Create, Delete};
use {ROOT_URL, STATIC_URL_ERROR};
use url::Url;
use super::{ApiLinks, ApiMeta};
use super::{HasValue, HasPagination, HasResponse};

const DOMAINS_SEGMENT: &'static str = "domains";

/// A domain name.
///
/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domains)
#[derive(Deserialize, Debug, Clone)]
pub struct Domain {
    /// The name of the domain itself. This should follow the standard domain
    /// format of domain.TLD. For instance, example.com is a valid domain name.
    pub name: String,
    /// This value is the time to live for the records on this domain, in 
    /// seconds. This defines the time frame that clients can cache queried
    /// information before a refresh should be requested.
    pub ttl: Option<usize>,
    /// This attribute contains the complete contents of the zone file for the
    /// selected domain. Individual domain record resources should be used to
    /// get more granular control over records. However, this attribute can 
    /// also be used to get information about the SOA record, which is created
    /// automatically and is not accessible as an individual record resource.
    pub zone_file: Option<String>,
}

impl Domain {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-domain)
    pub fn create<N, I>(name: N, ip_address: I) -> Request<Create, Domain>
    where N: AsRef<str> + Serialize + Display, I: Into<IpAddr> + Serialize + Display {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DOMAINS_SEGMENT);

        Request::new(url).body(json!({
            "name": name,
            "ip_address": ip_address,
        }))
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-domains)
    pub fn list() -> Request<List, Vec<Domain>> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DOMAINS_SEGMENT);

        Request::new(url)
    }

    /// Access [`DomainRecord`](struct.DomainRecord.html) types via [`.records()`](../request/struct.Request.html#method.records)
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-domain)
    pub fn get<N>(name: N) -> Request<Get, Domain> 
    where N: AsRef<str> + Display {        
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DOMAINS_SEGMENT)
            .push(name.as_ref());

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#delete-a-domain)
    pub fn delete<N>(name: N) -> Request<Delete, ()> 
    where N: AsRef<str> + Display {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DOMAINS_SEGMENT)
            .push(name.as_ref());
        
        Request::new(url)
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Debug, Clone)]
pub struct DomainsResponse {
    domain: Domain,
}

impl HasResponse for Domain {
    type Response = DomainsResponse;
}

impl HasValue for DomainsResponse {
    type Value = Domain;
    fn value(self) -> Domain {
        self.domain
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Debug, Clone)]
pub struct DomainsListResponse {
    domains: Vec<Domain>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasResponse for Vec<Domain> {
    type Response = DomainsListResponse;
}

impl HasPagination for DomainsListResponse {
    fn next_page(&self) -> Option<Url> {
        self.links.next()
    }
}

impl HasValue for DomainsListResponse {
    type Value = Vec<Domain>;
    fn value(self) -> Vec<Domain> {
        self.domains
    }
}