extern crate digitalocean;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
extern crate url;
extern crate url_serde;

mod utils;

use serde_json::Value;

use digitalocean::api::{Droplet, Snapshot};
use digitalocean::request::Request;
use digitalocean::method::{Get, List, Create, Delete};

use utils::before;

#[test]
fn create_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/droplets";
    let (name, region, size, image) = ("bear", "tor1", "5gb", "ubuntu-14-04-x64");
    let (ssh_keys, backups, monitoring) = (vec!["test", "test2"], true, true);

    let req: Request<Create, Droplet> = Droplet::create(name, region, size, image)
        .ssh_keys(ssh_keys.clone())
        .backups(backups)
        .monitoring(monitoring);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body,
               json!({
        "name": name,
        "region": region,
        "size": size,
        "image": image,
        "ssh_keys": ssh_keys,
        "backups": backups,
        "monitoring": monitoring,
    }));
}

#[test]
fn create_many_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/droplets";
    let (names, region, size, image) = (vec!["bear", "badger"], "tor1", "5gb", "ubuntu-14-04-x64");
    let (ssh_keys, backups, monitoring) = (vec!["test", "test2"], true, true);

    let req: Request<Create, Vec<Droplet>> =
        Droplet::create_multiple(names.clone(), region, size, image)
            .ssh_keys(ssh_keys.clone())
            .backups(backups)
            .monitoring(monitoring);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "names": names,
        "region": region,
        "size": size,
        "image": image,
        "ssh_keys": ssh_keys,
        "backups": backups,
        "monitoring": monitoring,
    }));
}

#[test]
fn get_produces_correct_request() {
    before();

    let droplet_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/droplets/{}", droplet_id);

    let req: Request<Get, Droplet> = Droplet::get(droplet_id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn list_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/droplets";

    let req: Request<List, Vec<Droplet>> = Droplet::list();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn list_by_tag_produces_correct_request() {
    before();

    let tag_name = "bear";
    let correct_url = format!("https://api.digitalocean.com/v2/droplets?tag_name={}", tag_name);

    let req: Request<List, Vec<Droplet>> = Droplet::list_by_tag("bear");
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn delete_produces_correct_request() {
    before();

    let droplet_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/droplets/{}", droplet_id);

    let req: Request<Delete, ()> = Droplet::delete(droplet_id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn delete_by_tag_produces_correct_request() {
    before();

    let tag_name = "bear";
    let correct_url = format!("https://api.digitalocean.com/v2/droplets?tag_name={}", tag_name);

    let req: Request<Delete, ()> = Droplet::delete_by_tag(tag_name);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn neighbors_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/reports/droplet_neighbors";

    let req: Request<Get, Vec<Vec<Droplet>>> = Droplet::neighbors();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn snapshots_produces_correct_request() {
    before();

    let droplet_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/droplets/{}/snapshots", droplet_id);

    let req: Request<List, Vec<Snapshot>> = Droplet::get(droplet_id).snapshots();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn backups_produces_correct_request() {
    before();

    let droplet_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/droplets/{}/backups", droplet_id);

    let req: Request<List, Vec<Snapshot>> = Droplet::get(droplet_id).backups();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn get_neighbors_produces_correct_request() {
    before();

    let droplet_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/droplets/{}/neighbors", droplet_id);

    let req: Request<List, Vec<Droplet>> = Droplet::get(droplet_id).neighbors();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}
