use serde::Serialize;
use std::fmt::Display;
use std::net::IpAddr;
use request::Request;
use action::{List, Get, Create, Delete};
use {ROOT_URL, STATIC_URL_ERROR};
use url::Url;
use values::Domain;
use super::{ApiLinks, ApiMeta};
use super::{HasValue, HasPagination};

const DOMAINS_SEGMENT: &'static str = "domains";

pub struct Domains;

impl Domains {
    /// https://developers.digitalocean.com/documentation/v2/#create-a-new-domain
    pub fn create<N, I>(name: N, ip_address: I) -> Request<Create, Domain>
    where N: AsRef<str> + Serialize + Display, I: Into<IpAddr> + Serialize + Display {
        info!("Creating {} ({}).", name, ip_address);
        
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DOMAINS_SEGMENT);

        let mut req = Request::new(url);
        req.body(json!({
                "name": name,
                "ip_address": ip_address,
            }));

        req
    }

    /// https://developers.digitalocean.com/documentation/v2/#list-all-domains
    pub fn list() -> Request<List, Vec<Domain>> {
        info!("Listing.");
        
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DOMAINS_SEGMENT);

        let req = Request::new(url);

        req
    }

    /// https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-domain
    pub fn get<N>(name: N) -> Request<Get, Domain> 
    where N: AsRef<str> + Display {
        info!("Getting {}.", name);
        
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DOMAINS_SEGMENT)
            .push(name.as_ref());

        let req = Request::new(url);

        req
    }

    /// https://developers.digitalocean.com/documentation/v2/#delete-a-domain
    pub fn delete<N>(name: N) -> Request<Delete, ()> 
    where N: AsRef<str> + Display {
        info!("Deleting {}.", name);
        
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DOMAINS_SEGMENT)
            .push(name.as_ref());
        
        let req = Request::new(url);

        req
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct DomainsListResponse {
    domains: Vec<Domain>,
    links: ApiLinks,
    meta: ApiMeta,
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

#[derive(Deserialize, Debug, Clone)]
pub struct DomainsResponse {
    domain: Domain,
}

impl HasValue for DomainsResponse {
    type Value = Domain;
    fn value(self) -> Domain {
        self.domain
    }
}