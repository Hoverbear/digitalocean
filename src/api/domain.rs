use serde::Serialize;
use serde_json;
use std::fmt::Display;
use std::io::Read;
use std::net::IpAddr;
use request::{Request, Method};
use DOMAINS_URL;
use Parse;
use error::Result;
use super::{ApiLinks, ApiMeta};

#[derive(Deserialize, Debug, Clone)]
pub struct Domain {
    pub name: String,
    pub ttl: Option<usize>,
    pub zone_file: Option<String>,
}

pub struct Domains;

impl Domains {
    pub fn create<'url, N,I>(name: N, ip_address: I) -> Request<'url, Domain>
    where N: AsRef<str> + Serialize + Display, I: Into<IpAddr> + Serialize + Display {
        info!("Creating {} ({}).", name, ip_address);
        let mut req = Request::new(Method::Post, &*DOMAINS_URL);

        req.body(json!({
                "name": name,
                "ip_address": ip_address,
            }));
        
        req
    }

    pub fn list<'url>(max: Option<usize>) -> Request<'url, Vec<Domain>> {
        info!("Listing.");
        let mut req = Request::new(Method::Get, &*DOMAINS_URL);
        
        req.paginated(true)
            .max_items(max);

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

impl Parse for Domain {
    fn parse<R>(reader: R) -> Result<Self> where R: Read {
        info!("Parsing.");
        let deserialized: DomainsResponse = serde_json::from_reader(reader)?;
        Ok(deserialized.domain)
    }
}

impl<'url> Request<'url, Domain> {
    pub fn set_zone<'a>(&'a mut self, val: String) -> &'a mut Self {
        info!("Buddy, I set the zone");
        panic!();
    }
}