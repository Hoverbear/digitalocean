extern crate digitalocean;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
extern crate url;
extern crate url_serde;

mod utils;

use serde_json::Value;

use digitalocean::api::Tag;
use digitalocean::request::Request;
use digitalocean::method::{Get, Create, Delete, List};

use utils::before;

#[test]
fn list_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/tags";

    let req: Request<List, Tag> = Tag::list();
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}

#[test]
fn get_produces_correct_request() {
    before();

    let tag = "test";
    let correct_url = format!("https://api.digitalocean.com/v2/tags/{}", tag);

    let req: Request<Get, Tag> = Tag::get(tag);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}

#[test]
fn create_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/tags";
    let tag = "test";

    let req: Request<Create, Tag> = Tag::create(tag);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(),
               json!({
        "name": tag,
    }));
}

#[test]
fn delete_produces_correct_request() {
    before();

    let tag = "test";
    let correct_url = format!("https://api.digitalocean.com/v2/tags/{}", tag);

    let req: Request<Delete, ()> = Tag::delete(tag);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}

#[test]
fn add_resources_produces_correct_request() {
    before();

    let tag = "test";
    let correct_url = format!("https://api.digitalocean.com/v2/tags/{}/resources", tag);
    let resources = vec![("123", "droplet"), ("456", "droplet")];

    let req: Request<Create, ()> = Tag::get(tag).add_resources(resources.clone());
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), json!({
        "resources": [
            {
                "resource_id": resources[0].0,
                "resource_type": resources[0].1,
            },
            {
                "resource_id": resources[1].0,
                "resource_type": resources[1].1,
            }
        ],
    }));
}

#[test]
fn remove_resources_produces_correct_request() {
    before();

    let tag = "test";
    let correct_url = format!("https://api.digitalocean.com/v2/tags/{}/resources", tag);
    let resources = vec![("123", "droplet"), ("456", "droplet")];

    let req: Request<Delete, ()> = Tag::get(tag).remove_resources(resources.clone());
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), json!({
        "resources": [
            {
                "resource_id": resources[0].0,
                "resource_type": resources[0].1,
            },
            {
                "resource_id": resources[1].0,
                "resource_type": resources[1].1,
            }
        ],
    }));
}
