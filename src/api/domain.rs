//! Domain specific documentation.

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

/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domains)
#[derive(Deserialize, Debug, Clone)]
pub struct Domain {
    pub name: String,
    pub ttl: Option<usize>,
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

#[derive(Deserialize, Debug, Clone)]
pub struct DomainsListResponse {
    domains: Vec<Domain>,
    links: ApiLinks,
    meta: ApiMeta,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DomainsResponse {
    domain: Domain,
}

impl HasResponse for Domain {
    type Response = DomainsResponse;
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

impl HasValue for DomainsResponse {
    type Value = Domain;
    fn value(self) -> Domain {
        self.domain
    }
}