extern crate digitalocean;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;
extern crate url;
extern crate url_serde;

mod utils;

use serde_json::Value;

use digitalocean::api::SshKeys;
use digitalocean::request::Request;
use digitalocean::values::SshKey;
use digitalocean::action::{Get, List, Create, Update, Delete};

use utils::before;

#[test]
fn list_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/account/keys";

    let req: Request<List, Vec<SshKey>> = SshKeys::list();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn create_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/account/keys";
    let (name, public_key) = ("test name", "test key");

    let req: Request<Create, SshKey> = SshKeys::create(name, public_key);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "name": name,
        "public_key": public_key,
    }));
}

#[test]
fn get_produces_correct_request() {
    before();

    let id = 123.to_string();
    let correct_url = format!("https://api.digitalocean.com/v2/account/keys/{}", id);

    let req: Request<Get, SshKey> = SshKeys::get(id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn update_produces_correct_request() {
    before();

    let id = 123.to_string();
    let correct_url = format!("https://api.digitalocean.com/v2/account/keys/{}", id);
    let name = "new name";

    let req: Request<Update, SshKey> = SshKeys::update(id)
        .name(name);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "name": name,
    }));
}

#[test]
fn delete_produces_correct_request() {
    before();

    let id = 123.to_string();
    let correct_url = format!("https://api.digitalocean.com/v2/account/keys/{}", id);

    let req: Request<Delete, ()> = SshKeys::delete(id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}