extern crate digitalocean;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
extern crate url;
extern crate url_serde;

mod utils;

use serde_json::Value;
use std::net::IpAddr;
use std::str::FromStr;

use digitalocean::api::{Action, FloatingIp};
use digitalocean::request::Request;
use digitalocean::method::{Get, Create, List};

use utils::before;

#[test]
fn list_produces_correct_request() {
    before();

    let floating_ip = IpAddr::from_str("192.168.0.1").unwrap();
    let correct_url = format!("https://api.digitalocean.com/v2/floating_ips/{}/actions",
                              floating_ip);

    let req: Request<List, Vec<Action>> = FloatingIp::get(floating_ip).actions();
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}

#[test]
fn get_produces_correct_request() {
    before();

    let floating_ip = IpAddr::from_str("192.168.0.1").unwrap();
    let action_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/floating_ips/{}/actions/{}",
                              floating_ip,
                              action_id);

    let req: Request<Get, Action> = FloatingIp::get(floating_ip).action(action_id);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}

#[test]
fn assign_produces_correct_request() {
    before();

    let floating_ip = IpAddr::from_str("192.168.0.1").unwrap();
    let correct_url = format!("https://api.digitalocean.com/v2/floating_ips/{}/actions",
                              floating_ip);
    let droplet_id = 123;

    let req: Request<Create, Action> = FloatingIp::get(floating_ip).assign(droplet_id);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(),
               json!({
        "type": "assign",
        "droplet_id": droplet_id
    }));
}

#[test]
fn unassign_produces_correct_request() {
    before();

    let floating_ip = IpAddr::from_str("192.168.0.1").unwrap();
    let correct_url =
        format!("https://api.digitalocean.com/v2/floating_ips/{}/actions", floating_ip);

    let req: Request<Create, Action> = FloatingIp::get(floating_ip).unassign();
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), json!({
        "type": "unassign",
    }));
}
