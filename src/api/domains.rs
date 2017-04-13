use serde::Serialize;
use std::fmt::Display;
use std::net::IpAddr;
use request::Request;
use action::{Get, Post, Delete};
use {ROOT_URL, STATIC_URL_ERROR};
use {HasValue};
use url::Url;
use super::{ApiLinks, ApiMeta, MAX_PER_PAGE};

const DOMAINS_SEGMENT: &'static str = "domains";

pub struct Domains;

#[derive(Deserialize, Debug, Clone)]
pub struct Domain {
    pub name: String,
    pub ttl: Option<usize>,
    pub zone_file: Option<String>,
}

impl Domains {
    pub fn create<N, I>(name: N, ip_address: I) -> Request<Post, DomainsResponse>
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

    pub fn list() -> Request<Get, DomainsListResponse> {
        info!("Listing.");
        
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DOMAINS_SEGMENT);
        url.query_pairs_mut()
            .append_pair("per_page", &MAX_PER_PAGE.to_string());

        let req = Request::new(url);

        req
    }

    pub fn get<N>(name: N) -> Request<Get, DomainsResponse> 
    where N: AsRef<str> + Display {
        info!("Getting {}.", name);
        
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DOMAINS_SEGMENT)
            .push(name.as_ref());
        url.query_pairs_mut()
            .append_pair("per_page", &MAX_PER_PAGE.to_string());

        let req = Request::new(url);

        req
    }

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

impl HasValue for DomainsListResponse {
    type Value = Vec<Domain>;
    fn next_page(&self) -> Option<Url> {
        match self.links.pages {
            Some(ref pages) => match pages.next {
                Some(ref v) => Some(v.clone().into_inner()),
                None => None,
            },
            None => None,
        }
    }
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
    fn next_page(&self) -> Option<Url> { None }
    fn value(self) -> Domain {
        self.domain
    }
}