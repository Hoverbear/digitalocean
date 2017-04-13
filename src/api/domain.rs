use serde::Serialize;
use std::fmt::Display;
use std::net::IpAddr;
use request::Request;
use method::{Get, Post, Delete};
use {ROOT_URL, STATIC_URL_ERROR};
use {Retrievable, DigitalOcean};
use url::Url;
use error::Result;
use super::{ApiLinks, ApiMeta, MAX_PER_PAGE};

const DOMAINS_SEGMENT: &'static str = "domains";

#[derive(Deserialize, Debug, Clone)]
pub struct Domain {
    pub name: String,
    pub ttl: Option<usize>,
    pub zone_file: Option<String>,
}

pub struct Domains;

impl Domains {
    pub fn create<N, I>(name: N, ip_address: I) -> Request<Post, Domain>
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

    pub fn list() -> Request<Get, Vec<Domain>> {
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

    pub fn get<N>(name: N) -> Request<Get, Domain> 
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

    pub fn delete<N>(name: N) -> Request<Delete, Domain> 
    where N: AsRef<str> + Display {
        info!("Deleting {}.", name);
        
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DOMAINS_SEGMENT)
            .push(name.as_ref());
        warn!("URL IS {}", url);
        let req = Request::new(url);

        req
    }
}

#[derive(Deserialize, Debug)]
struct DomainsListResponse {
    domains: Vec<Domain>,
    links: ApiLinks,
    meta: ApiMeta,
}

#[derive(Deserialize, Debug)]
struct DomainsResponse {
    domain: Domain,
}

impl Retrievable<Domain> for Request<Get, Domain> {
    fn retrieve(self, instance: &DigitalOcean) -> Result<Domain> {
        info!("Retrieving GET.");
        let response: DomainsResponse = instance.get(self)?;
        Ok(response.domain)
    }
}

impl Retrievable<Domain> for Request<Post, Domain> {
    fn retrieve(self, instance: &DigitalOcean) -> Result<Domain> {
        info!("Retrieving POST.");
        let response: DomainsResponse = instance.post(self)?;
        Ok(response.domain)
    }
}

impl Retrievable<()> for Request<Delete, Domain> {
    fn retrieve(self, instance: &DigitalOcean) -> Result<()> {
        info!("Retrieving DELETE.");
        let response = instance.delete(self)?;
        Ok(response)
    }
}

impl Retrievable<Vec<Domain>> for Request<Get, Vec<Domain>> {
    fn retrieve(self, instance: &DigitalOcean) -> Result<Vec<Domain>> {
        info!("Retrieving GET.");
        // This is a paginated response. We need to buffer.
        let mut buffer = Vec::new();
        let mut current_url = self.url.clone();

        loop {
            let deserialized: DomainsListResponse = {
                let mut current_request = self.clone();
                current_request.url = current_url;
                instance.get(current_request)?
            };
            buffer.extend(deserialized.domains);
            current_url = match deserialized.links.pages {
                Some(pages) => match pages.next {
                    Some(val) => Url::parse(&val)?,
                    None => break,
                },
                None => break,
            };
        }

        Ok(buffer)
    }
}

impl Request<Get, Domain> {
    pub fn records<'a>(&'a mut self) -> &'a mut Self {
        
        self
    }
}