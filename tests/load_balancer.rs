extern crate digitalocean;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
extern crate url;
extern crate url_serde;

mod utils;

use serde_json::Value;

use digitalocean::api::LoadBalancer;
use digitalocean::method::{Create, Delete, Get, List, Update};
use digitalocean::request::Request;

use utils::before;

#[test]
fn list_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/load_balancers";

    let req: Request<List, Vec<LoadBalancer>> = LoadBalancer::list();
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}

#[test]
fn get_produces_correct_request() {
    before();

    let load_balancer_id = "123";
    let correct_url = format!(
        "https://api.digitalocean.com/v2/load_balancers/{}",
        load_balancer_id
    );

    let req: Request<Get, LoadBalancer> = LoadBalancer::get(load_balancer_id);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}

#[test]
fn create_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/load_balancers";
    let (name, region, algo) = ("test", "tor1", "least_connections");
    let rule_1 = ("tcp", 22, "tcp", 22);
    let rule_2 = ("http", 443, "http", 443, None, false);

    let req: Request<Create, LoadBalancer> = LoadBalancer::create(name, region)
        .algorithm(algo)
        .forwarding_rule(rule_1)
        .forwarding_rule(rule_2);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(
        *req.body(),
        json!({
        "name": name,
        "region": region,
        "algorithm": algo,
        "forwarding_rules": [
            {
                "entry_protocol": rule_1.0,
                "entry_port": rule_1.1,
                "target_protocol": rule_1.2,
                "target_port": rule_1.3,
                "certificate_id": Value::Null,
                "tls_passthrough": false,
            },
            {
                "entry_protocol": rule_2.0,
                "entry_port": rule_2.1,
                "target_protocol": rule_2.2,
                "target_port": rule_2.3,
                "certificate_id": Value::Null,
                "tls_passthrough": false,
            },
        ],
    })
    );
}

#[test]
fn update_produces_correct_request() {
    before();

    let load_balancer_id = "123";
    let correct_url = format!(
        "https://api.digitalocean.com/v2/load_balancers/{}",
        load_balancer_id
    );
    let (name, region, tag) = ("test", "tor1", "tag");
    let (e_protocol, e_port, t_protocol, t_port) =
        (String::from("tcp"), 22, String::from("tcp"), 22);

    let req: Request<Update, LoadBalancer> = LoadBalancer::update(load_balancer_id)
        .name(name)
        .region(region)
        .tag(tag)
        .forwarding_rule((e_protocol.clone(), e_port, t_protocol.clone(), t_port));
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(
        *req.body(),
        json!({
        "name": name,
        "region": region,
        "tag": tag,
        "forwarding_rules": [
            {
                "entry_protocol": e_protocol,
                "entry_port": e_port,
                "target_protocol": t_protocol,
                "target_port": t_port,
                "certificate_id": Value::Null,
                "tls_passthrough": false,
            },
        ],
    })
    );
}

#[test]
fn delete_produces_correct_request() {
    before();

    let load_balancer_id = "123";
    let correct_url = format!(
        "https://api.digitalocean.com/v2/load_balancers/{}",
        load_balancer_id
    );

    let req: Request<Delete, ()> = LoadBalancer::delete(load_balancer_id);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}

#[test]
fn add_droplets_produces_correct_request() {
    before();

    let load_balancer_id = "123";
    let correct_url = format!(
        "https://api.digitalocean.com/v2/load_balancers/{}/droplets",
        load_balancer_id
    );
    let droplet_ids = vec![123, 456, 789];

    let req: Request<Create, ()> =
        LoadBalancer::get(load_balancer_id).add_droplets(droplet_ids.clone());
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(
        *req.body(),
        json!({
        "droplet_ids": droplet_ids,
    })
    );
}

#[test]
fn remove_droplets_produces_correct_request() {
    before();

    let load_balancer_id = "123";
    let correct_url = format!(
        "https://api.digitalocean.com/v2/load_balancers/{}/droplets",
        load_balancer_id
    );
    let droplet_ids = vec![123, 456, 789];

    let req: Request<Delete, ()> =
        LoadBalancer::get(load_balancer_id).remove_droplets(droplet_ids.clone());
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(
        *req.body(),
        json!({
        "droplet_ids": droplet_ids,
    })
    );
}

#[test]
fn add_forwarding_rule_produces_correct_request() {
    before();

    let load_balancer_id = "123";
    let correct_url = format!(
        "https://api.digitalocean.com/v2/load_balancers/{}/forwarding_rules",
        load_balancer_id
    );
    let (e_protocol, e_port, t_protocol, t_port) = ("tcp", 22, "tcp", 22);

    let req: Request<Create, ()> = LoadBalancer::get(load_balancer_id)
        .add_forwarding_rules(vec![(e_protocol, e_port, t_protocol, t_port, None, true)]);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(
        *req.body(),
        json!({
        "forwarding_rules": [
            {
                "entry_protocol": e_protocol,
                "entry_port": e_port,
                "target_protocol": t_protocol,
                "target_port": t_port,
                "certificate_id": Value::Null,
                "tls_passthrough": true,
            },
        ],
    })
    );
}

#[test]
fn remove_forwarding_rule_produces_correct_request() {
    before();

    let load_balancer_id = "123";
    let correct_url = format!(
        "https://api.digitalocean.com/v2/load_balancers/{}/forwarding_rules",
        load_balancer_id
    );
    let (e_protocol, e_port, t_protocol, t_port) = ("tcp", 22, "tcp", 22);

    let req: Request<Delete, ()> = LoadBalancer::get(load_balancer_id)
        .remove_forwarding_rules(vec![(e_protocol, e_port, t_protocol, t_port, None, false)]);
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(
        *req.body(),
        json!({
        "forwarding_rules": [
            {
                "entry_protocol": e_protocol,
                "entry_port": e_port,
                "target_protocol": t_protocol,
                "target_port": t_port,
                "certificate_id": Value::Null,
                "tls_passthrough": false,
            },
        ],
    })
    );
}
