extern crate digitalocean;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
extern crate url;
extern crate url_serde;

mod utils;

use serde_json::Value;

use digitalocean::api::SshKey;
use digitalocean::request::Request;
use digitalocean::method::{Get, List, Create, Update, Delete};

use utils::before;

#[test]
fn list_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/account/keys";

    let req: Request<List, Vec<SshKey>> = SshKey::list();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn create_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/account/keys";
    let (name, public_key) = ("test name", "test key");

    let req: Request<Create, SshKey> = SshKey::create(name, public_key);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body,
               json!({
        "name": name,
        "public_key": public_key,
    }));
}

#[test]
fn get_produces_correct_request() {
    before();

    let key_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/account/keys/{}", key_id);

    let req: Request<Get, SshKey> = SshKey::get(key_id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn update_produces_correct_request() {
    before();

    let key_id = 123.to_string();
    let correct_url = format!("https://api.digitalocean.com/v2/account/keys/{}", key_id);
    let name = "new name";

    let req: Request<Update, SshKey> = SshKey::update(key_id).name(name);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "name": name,
    }));
}

#[test]
fn delete_produces_correct_request() {
    before();

    let key_id = 123.to_string();
    let correct_url = format!("https://api.digitalocean.com/v2/account/keys/{}", key_id);

    let req: Request<Delete, ()> = SshKey::delete(key_id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}
