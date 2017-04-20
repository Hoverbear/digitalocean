extern crate digitalocean;
#[macro_use] extern crate log;
extern crate serde_json;
extern crate url;
extern crate url_serde;

mod utils;

use serde_json::Value;

use digitalocean::api::Snapshot;
use digitalocean::request::Request;
use digitalocean::action::{Get, List, Delete};

use utils::before;

#[test]
fn list_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/snapshots";

    let req: Request<List, Vec<Snapshot>> = Snapshot::list();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn droplets_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/snapshots?resource_type=droplet";

    let req: Request<List, Vec<Snapshot>> = Snapshot::droplets();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn volumes_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/snapshots?resource_type=volume";

    let req: Request<List, Vec<Snapshot>> = Snapshot::volumes();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn get_produces_correct_request() {
    before();

    let snapshot_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/snapshots/{}", snapshot_id);

    let req: Request<Get, Snapshot> = Snapshot::get(snapshot_id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn delete_produces_correct_request() {
    before();

    let snapshot_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/snapshots/{}", snapshot_id);

    let req: Request<Delete, ()> = Snapshot::delete(snapshot_id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}