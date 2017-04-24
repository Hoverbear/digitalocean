use serde::Serialize;
use std::fmt::Display;
use request::Request;
use method::{Create, Delete, Get, List};
use super::{Region, Droplet};
use {ROOT_URL, STATIC_URL_ERROR};
use url::Url;
use std::net::IpAddr;
use super::{ApiLinks, ApiMeta};
use super::{HasValue, HasPagination, HasResponse};

const FLOATING_IP_SEGMENT: &'static str = "floating_ips";

/// Floating IP objects represent a publicly-accessible static IP addresses 
/// that can be mapped to one of your Droplets. They can be used to create 
/// highly available setups or other configurations requiring movable 
/// addresses.
///
/// Floating IPs are bound to a specific region.
///
/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#floating-ips)
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FloatingIp {
    /// The public IP address of the Floating IP. It also serves as its 
    /// identifier.
    pub ip: IpAddr,
    /// The region that the Floating IP is reserved to. When you query a 
    /// Floating IP, the entire region object will be returned.
    pub region: Region,
    /// The Droplet that the Floating IP has been assigned to. When you query
    /// a Floating IP, if it is assigned to a Droplet, the entire Droplet 
    /// object will be returned. If it is not assigned, the value will be null.
    pub droplet: Option<Droplet>,
}

impl FloatingIp {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-floating-ips)
    pub fn list() -> Request<List, Vec<FloatingIp>> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(FLOATING_IP_SEGMENT);

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-floating-ip-assigned-to-a-droplet)
    pub fn for_droplet(id: usize) -> Request<Create, FloatingIp> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(FLOATING_IP_SEGMENT);

        Request::new(url).body(json!({
            "droplet_id": id,
        }))
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-floating-ip-reserved-to-a-region)
    pub fn for_region<S>(id: S) -> Request<Create, FloatingIp>
    where S: AsRef<str> + Display + Serialize {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(FLOATING_IP_SEGMENT);

        Request::new(url).body(json!({
            "region": id,
        }))
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-floating-ip)
    pub fn get<I>(id: I) -> Request<Get, FloatingIp>
    where I: Into<IpAddr> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(FLOATING_IP_SEGMENT)
            .push(&id.into().to_string());

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-floating-ip)
    pub fn delete<I>(id: I) -> Request<Delete, ()>
    where I: Into<IpAddr> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(FLOATING_IP_SEGMENT)
            .push(&id.into().to_string());

        Request::new(url)
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FloatingIpResponse {
    floating_ip: FloatingIp,
}

impl HasResponse for FloatingIp {
    type Response = FloatingIpResponse;
}

impl HasValue for FloatingIpResponse {
    type Value = FloatingIp;
    fn value(self) -> FloatingIp {
        self.floating_ip
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FloatingIpListResponse {
    floating_ips: Vec<FloatingIp>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasResponse for Vec<FloatingIp> {
    type Response = FloatingIpListResponse;
}

impl HasPagination for FloatingIpListResponse {
    fn next_page(&self) -> Option<Url> {
        self.links.next()
    }
}

impl HasValue for FloatingIpListResponse {
    type Value = Vec<FloatingIp>;
    fn value(self) -> Vec<FloatingIp> {
        self.floating_ips
    }
}
