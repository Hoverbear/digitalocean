extern crate digitalocean;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;
extern crate url;
extern crate url_serde;

mod utils;

use serde_json::Value;

use digitalocean::api::LoadBalancer;
use digitalocean::request::Request;
use digitalocean::method::{Get, List, Update, Create, Delete};

use utils::before;

#[test]
fn list_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/load_balancers";

    let req: Request<List, Vec<LoadBalancer>> = LoadBalancer::list();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}


#[test]
fn get_produces_correct_request() {
    before();

    let load_balancer_id = "123";
    let correct_url = format!("https://api.digitalocean.com/v2/load_balancers/{}", load_balancer_id);

    let req: Request<Get, LoadBalancer> = LoadBalancer::get(load_balancer_id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn create_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/load_balancers";
    let (name, region, algo) = ("test", "tor1", "least_connections");
    let (e_protocol, e_port, t_protocol, t_port) = ("tcp", 22, "tcp", 22);
    let (e_protocol_2, e_port_2, t_protocol_2, t_port_2) = ("http", 443, "http", 443);

    let req: Request<Create, LoadBalancer> = LoadBalancer::create(name, region)
        .algorithm(algo)
        .forwarding_rule(
            e_protocol.clone(),
            e_port,
            t_protocol.clone(),
            t_port,
            None,
            None)
        .forwarding_rule(
            e_protocol_2.clone(),
            e_port_2,
            t_protocol_2.clone(),
            t_port_2,
            None,
            None);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "name": name,
        "region": region,
        "algorithm": algo,
        "forwarding_rules": [
            {
                "entry_protocol": e_protocol,
                "entry_port": e_port,
                "target_protocol": t_protocol,
                "target_port": t_port,
            },
            {
                "entry_protocol": e_protocol_2,
                "entry_port": e_port_2,
                "target_protocol": t_protocol_2,
                "target_port": t_port_2,
            },
        ],
    }));
}

#[test]
fn update_produces_correct_request() {
    before();

    let load_balancer_id = "123";
    let correct_url = format!("https://api.digitalocean.com/v2/load_balancers/{}", load_balancer_id);
    let (name, region, tag) = ("test", "tor1", "tag");
    let (e_protocol, e_port, t_protocol, t_port) = ("tcp", 22, "tcp", 22);

    let req: Request<Update, LoadBalancer> = LoadBalancer::update(load_balancer_id)
        .name(name)
        .region(region)
        .tag(tag)
        .forwarding_rule(
            e_protocol.clone(),
            e_port,
            t_protocol.clone(),
            t_port,
            None,
            None);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "name": name,
        "region": region,
        "tag": tag,
        "forwarding_rules": [
            {
                "entry_protocol": e_protocol,
                "entry_port": e_port,
                "target_protocol": t_protocol,
                "target_port": t_port,
            },
        ],
    }));
}

#[test]
fn delete_produces_correct_request() {
    before();

    let load_balancer_id = "123";
    let correct_url = format!("https://api.digitalocean.com/v2/load_balancers/{}", load_balancer_id);

    let req: Request<Delete, ()> = LoadBalancer::delete(load_balancer_id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn add_droplets_produces_correct_request() {
    before();

    let load_balancer_id = "123";
    let correct_url = format!("https://api.digitalocean.com/v2/load_balancers/{}/droplets", load_balancer_id);
    let droplet_ids = vec![123, 456, 789];

    let req: Request<Create, ()> = LoadBalancer::get(load_balancer_id)
        .add_droplets(droplet_ids.clone());
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "droplet_ids": droplet_ids,
    }));
}

#[test]
fn remove_droplets_produces_correct_request() {
    before();

    let load_balancer_id = "123";
    let correct_url = format!("https://api.digitalocean.com/v2/load_balancers/{}/droplets", load_balancer_id);
    let droplet_ids = vec![123, 456, 789];

    let req: Request<Delete, ()> = LoadBalancer::get(load_balancer_id)
        .remove_droplets(droplet_ids.clone());
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "droplet_ids": droplet_ids,
    }));
}

#[test]
fn add_forwarding_rule_produces_correct_request() {
    before();

    let load_balancer_id = "123";
    let correct_url = format!("https://api.digitalocean.com/v2/load_balancers/{}/forwarding_rules", load_balancer_id);
    let (e_protocol, e_port, t_protocol, t_port) = ("tcp", 22, "tcp", 22);

    let req: Request<Create, ()> = LoadBalancer::get(load_balancer_id)
        .add_forwarding_rule(e_protocol, e_port, t_protocol, t_port, None, None);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "forwarding_rules": [
            {
                "entry_protocol": e_protocol,
                "entry_port": e_port,
                "target_protocol": t_protocol,
                "target_port": t_port,
            },
        ],
    }));
}

#[test]
fn remove_forwarding_rule_produces_correct_request() {
    before();

    let load_balancer_id = "123";
    let correct_url = format!("https://api.digitalocean.com/v2/load_balancers/{}/forwarding_rules", load_balancer_id);
    let (e_protocol, e_port, t_protocol, t_port) = ("tcp", 22, "tcp", 22);

    let req: Request<Delete, ()> = LoadBalancer::get(load_balancer_id)
        .remove_forwarding_rule(e_protocol, e_port, t_protocol, t_port, None, None);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "forwarding_rules": [
            {
                "entry_protocol": e_protocol,
                "entry_port": e_port,
                "target_protocol": t_protocol,
                "target_port": t_port,
            },
        ],
    }));
}