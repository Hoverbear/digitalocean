extern crate digitalocean;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;
extern crate url;
extern crate url_serde;

mod utils;

use serde_json::Value;
use std::net::IpAddr;
use std::str::FromStr;

use digitalocean::api::FloatingIp;
use digitalocean::request::Request;
use digitalocean::action::{Create, Delete, Get, List};

use utils::before;

#[test]
fn list_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/floating_ips";

    let req: Request<List, Vec<FloatingIp>> = FloatingIp::list();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn for_droplet_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/floating_ips";
    let droplet_id = 123;

    let req: Request<Create, FloatingIp> = FloatingIp::for_droplet(droplet_id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "droplet_id": droplet_id,
    }));
}

#[test]
fn for_region_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/floating_ips";
    let region_id = "tor1";

    let req: Request<Create, FloatingIp> = FloatingIp::for_region(region_id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "region": region_id,
    }));
}

#[test]
fn get_produces_correct_request() {
    before();

    let floating_ip = IpAddr::from_str("192.168.0.1").unwrap();
    let correct_url = format!("https://api.digitalocean.com/v2/floating_ips/{}", floating_ip);

    let req: Request<Get, FloatingIp> = FloatingIp::get(floating_ip);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn delete_produces_correct_request() {
    before();

    let floating_ip = IpAddr::from_str("192.168.0.1").unwrap();
    let correct_url = format!("https://api.digitalocean.com/v2/floating_ips/{}", floating_ip);

    let req: Request<Delete, ()> = FloatingIp::delete(floating_ip);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}
