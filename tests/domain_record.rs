extern crate digitalocean;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
extern crate url;
extern crate url_serde;

mod utils;

use serde_json::Value;

use digitalocean::api::{Domain, DomainRecord};
use digitalocean::method::{Create, Delete, Get, List, Update};
use digitalocean::request::Request;

use crate::utils::before;

#[test]
fn list_produces_correct_request() {
    before();

    let domain = "example.com";
    let correct_url = format!("https://api.digitalocean.com/v2/domains/{}/records", domain);

    let req: Request<List, Vec<DomainRecord>> = Domain::get(domain).records();
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}

#[test]
fn create_produces_correct_request() {
    before();

    let domain = "example.com";
    let correct_url = format!("https://api.digitalocean.com/v2/domains/{}/records", domain);
    let (kind, name, data, ttl) = ("A", "www", "192.168.0.1", 100);

    let req: Request<Create, DomainRecord> = Domain::get(domain)
        .records()
        .create(kind, name, data)
        .ttl(ttl);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(
        *req.body(),
        json!({
            "type": kind,
            "name": name,
            "data": data,
            "ttl": ttl,
        })
    );
}

#[test]
fn get_produces_correct_request() {
    before();

    let domain = "example.com";
    let record_id = 123;
    let correct_url = format!(
        "https://api.digitalocean.com/v2/domains/{}/records/{}",
        domain, record_id
    );

    let req: Request<Get, DomainRecord> = Domain::get(domain).records().get(record_id);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}

#[test]
fn update_produces_correct_request() {
    before();

    let domain = "example.com";
    let record_id = 123;
    let correct_url = format!(
        "https://api.digitalocean.com/v2/domains/{}/records/{}",
        domain, record_id
    );
    let (kind, name, ttl) = ("SRV", "ww2", 200);

    let req: Request<Update, DomainRecord> = Domain::get(domain)
        .records()
        .update(record_id)
        .kind(kind)
        .name(name)
        .ttl(ttl);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(
        *req.body(),
        json!({
            "type": kind,
            "name": name,
            "ttl": ttl,
        })
    );
}

#[test]
fn delete_produces_correct_request() {
    before();

    let domain = "example.com";
    let record_id = 123;
    let correct_url = format!(
        "https://api.digitalocean.com/v2/domains/{}/records/{}",
        domain, record_id
    );

    let req: Request<Delete, ()> = Domain::get(domain).records().delete(record_id);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}
