extern crate digitalocean;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
extern crate url;
extern crate url_serde;

mod utils;

use serde_json::Value;

use digitalocean::api::{Volume, Snapshot};
use digitalocean::request::Request;
use digitalocean::method::{Get, List, Create, Delete};

use utils::before;


#[test]
fn list_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/volumes";

    let req: Request<List, Vec<Volume>> = Volume::list();
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);

    // With region
    let region = "tor1";
    let correct_url = format!("https://api.digitalocean.com/v2/volumes?region={}", region);

    let req: Request<List, Vec<Volume>> = Volume::list().region(region);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}


#[test]
fn create_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/volumes";
    let (name, size, region) = ("bear", 123, String::from("tor1"));

    let req: Request<Create, Volume> = Volume::create(name, size).region(region.clone());
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(
        *req.body(),
        json!({
        "name": name,
        "size_gigabytes": size,
        "region": region,
    })
    );
}


#[test]
fn get_produces_correct_request() {
    before();

    let volume_id = "123";
    let correct_url = format!("https://api.digitalocean.com/v2/volumes/{}", volume_id);

    let req: Request<Get, Volume> = Volume::get(volume_id);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}

#[test]
fn get_by_name_produces_correct_request() {
    before();

    let name = "test";
    let region = "tor1";
    let correct_url = format!("https://api.digitalocean.com/v2/volumes?name={}&region={}", name, region);

    let req: Request<Get, Volume> = Volume::get_by_name(name, region);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}

#[test]
fn delete_produces_correct_request() {
    before();

    let volume_id = "123";
    let correct_url = format!("https://api.digitalocean.com/v2/volumes/{}", volume_id);

    let req: Request<Delete, ()> = Volume::delete(volume_id);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}

#[test]
fn delete_by_name_produces_correct_request() {
    before();

    let name = "test";
    let region = "tor1";
    let correct_url = format!("https://api.digitalocean.com/v2/volumes?name={}&region={}", name, region);

    let req: Request<Delete, ()> = Volume::delete_by_name(name, region);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}

#[test]
fn snapshots_produces_correct_request() {
    before();

    let volume_id = "123";
    let correct_url = format!("https://api.digitalocean.com/v2/volumes/{}/snapshots", volume_id);

    let req: Request<List, Vec<Snapshot>> = Volume::get(volume_id).snapshots();
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}

#[test]
fn snapshot_produces_correct_request() {
    before();

    let volume_id = "123";
    let correct_url = format!("https://api.digitalocean.com/v2/volumes/{}/snapshots", volume_id);
    let snapshot_name = "test";

    let req: Request<Create, Snapshot> = Volume::get(volume_id).snapshot(snapshot_name);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), json!({
        "name": snapshot_name
    }));
}
