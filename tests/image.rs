extern crate digitalocean;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
extern crate url;
extern crate url_serde;

mod utils;

use serde_json::Value;

use digitalocean::api::Image;
use digitalocean::request::Request;
use digitalocean::method::{Get, Update, Delete, List};

use utils::before;

#[test]
fn list_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/images";

    let req: Request<List, Vec<Image>> = Image::list();
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}

#[test]
fn get_produces_correct_request() {
    before();

    let image_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/images/{}", image_id);

    let req: Request<Get, Image> = Image::get(image_id);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}

#[test]
fn update_produces_correct_request() {
    before();

    let image_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/images/{}", image_id);
    let name = "blah-blah";

    let req: Request<Update, Image> = Image::update(image_id).name(name);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(),
               json!({
        "name": name,
    }));
}

#[test]
fn delete_produces_correct_request() {
    before();

    let image_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/images/{}", image_id);

    let req: Request<Delete, ()> = Image::delete(image_id);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}
