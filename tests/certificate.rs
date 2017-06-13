extern crate digitalocean;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
extern crate url;
extern crate url_serde;

mod utils;

use serde_json::Value;

use digitalocean::api::Certificate;
use digitalocean::request::Request;
use digitalocean::method::{Get, List, Create, Delete};

use utils::before;

#[test]
fn list_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/certificates";

    let req: Request<List, Vec<Certificate>> = Certificate::list();
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}

#[test]
fn create_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/certificates";
    let (name, pkey, leaf, chain) =
        (String::from("test"), String::from("pkey"), String::from("leaf"), String::from("chain"));


    let req: Request<Create, Certificate> =
        Certificate::create(name.clone(), pkey.clone(), leaf.clone())
            .certificate_chain(chain.clone());
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(),
               json!({
        "name": name,
        "private_key": pkey,
        "leaf_certificate": leaf,
        "certificate_chain": chain,
    }));
}

#[test]
fn get_produces_correct_request() {
    before();

    let certificate_id = "123";
    let correct_url = format!("https://api.digitalocean.com/v2/certificates/{}", certificate_id);

    let req: Request<Get, Certificate> = Certificate::get(certificate_id);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}

#[test]
fn delete_produces_correct_request() {
    before();

    let certificate_id = "123";
    let correct_url = format!("https://api.digitalocean.com/v2/certificates/{}", certificate_id);

    let req: Request<Delete, ()> = Certificate::delete(certificate_id);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}
