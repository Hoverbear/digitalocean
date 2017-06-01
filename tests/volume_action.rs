extern crate digitalocean;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
extern crate url;
extern crate url_serde;

mod utils;

use serde_json::Value;

use digitalocean::api::{Volume, Action};
use digitalocean::request::Request;
use digitalocean::method::{Get, List, Create};

use utils::before;

#[test]
fn attach_produces_correct_request() {
    before();

    let volume_id = "123";
    let correct_url = format!("https://api.digitalocean.com/v2/volumes/{}/actions",
                              volume_id);
    let droplet_id = 456;

    let req: Request<Create, Action> = Volume::get(volume_id).attach(droplet_id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body,
               json!({
        "type": "attach",
        "droplet_id": droplet_id,
    }));
}

#[test]
fn attach_by_name_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/volumes";
    let volume_id = "123";
    let droplet_id = 456;

    let req: Request<Create, Action> = Volume::attach(volume_id, droplet_id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": "attach",
        "droplet_id": droplet_id,
        "volume_name": volume_id
    }));
}

#[test]
fn detach_produces_correct_request() {
    before();

    let volume_id = "123";
    let correct_url = format!("https://api.digitalocean.com/v2/volumes/{}/actions", volume_id);
    let droplet_id = 456;

    let req: Request<Create, Action> = Volume::get(volume_id).detach(droplet_id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": "detach",
        "droplet_id": droplet_id,
    }));
}

#[test]
fn detach_by_name_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/volumes";
    let volume_id = "123";
    let droplet_id = 456;

    let req: Request<Create, Action> = Volume::detach(volume_id, droplet_id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": "detach",
        "droplet_id": droplet_id,
        "volume_name": volume_id
    }));
}

#[test]
fn resize_produces_correct_request() {
    before();

    let volume_id = "123";
    let correct_url = format!("https://api.digitalocean.com/v2/volumes/{}/actions", volume_id);

    let req: Request<Create, Action> = Volume::get(volume_id).resize(123);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": "resize",
        "size_gigabytes": 123,
    }));
}

#[test]
fn list_produces_correct_request() {
    before();

    let volume_id = "123";
    let correct_url = format!("https://api.digitalocean.com/v2/volumes/{}/actions", volume_id);

    let req: Request<List, Vec<Action>> = Volume::get(volume_id).actions();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn get_produces_correct_request() {
    before();

    let volume_id = "123";
    let action_id = 456;
    let correct_url =
        format!("https://api.digitalocean.com/v2/volumes/{}/actions/{}", volume_id, action_id);

    let req: Request<Get, Action> = Volume::get(volume_id).action(action_id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}
