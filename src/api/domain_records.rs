use serde::Serialize;
use std::fmt::Display;
use std::net::IpAddr;
use request::Request;
use action::{Get, Post, Delete};
use {ROOT_URL, STATIC_URL_ERROR};
use {HasValue};
use url::Url;
use super::{ApiLinks, ApiMeta, MAX_PER_PAGE};
use super::domains::DomainsResponse;


#[derive(Deserialize, Debug, Clone)]
pub struct DomainRecordsResponse {
    domain_records: Vec<DomainRecord>,
    links: ApiLinks,
    meta: ApiMeta,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DomainRecord {
    id: usize,
    #[serde(rename = "type")]
    kind: String, // 'type' is reserved in Rust.
    name: String,
    data: String,
    priority: Option<usize>,
    port: Option<usize>,
    weight: Option<usize>,
    
}

impl HasValue for DomainRecordsResponse {
    type Value = Vec<DomainRecord>;
    fn next_page(&self) -> Option<Url> {
        match self.links.pages {
            Some(ref pages) => match pages.next {
                Some(ref v) => Some(v.clone().into_inner()),
                None => None,
            },
            None => None,
        }
    }
    fn value(self) -> Vec<DomainRecord> {
        self.domain_records
    }
}

impl Request<Get, DomainsResponse> {
    pub fn records<'a>(&'a mut self) -> &'a mut Request<Get, DomainRecordsResponse> {
        self.url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push("records");
        // Safe because we're only changing PhantomData.
        unsafe {
            &mut *(self as *mut _ as *mut Request<Get, DomainRecordsResponse>)
        }
    }
}