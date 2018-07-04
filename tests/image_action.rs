extern crate digitalocean;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
extern crate url;
extern crate url_serde;

mod utils;

use serde_json::Value;

use digitalocean::api::{Action, Image};
use digitalocean::method::{Create, Get, List};
use digitalocean::request::Request;

use utils::before;

#[test]
fn list_produces_correct_request() {
    before();

    let image_id = 123;
    let correct_url = format!(
        "https://api.digitalocean.com/v2/images/{}/actions",
        image_id
    );

    let req: Request<List, Vec<Action>> = Image::get(image_id).actions();
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}

#[test]
fn get_produces_correct_request() {
    before();

    let image_id = 123;
    let action_id = 456;
    let correct_url = format!(
        "https://api.digitalocean.com/v2/images/{}/actions/{}",
        image_id, action_id
    );

    let req: Request<Get, Action> = Image::get(image_id).action(action_id);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}

#[test]
fn transfer_produces_correct_request() {
    before();

    let image_id = 123;
    let correct_url = format!(
        "https://api.digitalocean.com/v2/images/{}/actions",
        image_id
    );
    let region = "tor1";

    let req: Request<Create, Action> = Image::get(image_id).transfer(region);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(
        *req.body(),
        json!({
        "type": "transfer",
        "region": region,
    })
    );
}

#[test]
fn convert_produces_correct_request() {
    before();

    let image_id = 123;
    let correct_url = format!(
        "https://api.digitalocean.com/v2/images/{}/actions",
        image_id
    );

    let req: Request<Create, Action> = Image::get(image_id).convert();
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(
        *req.body(),
        json!({
        "type": "convert",
    })
    );
}
