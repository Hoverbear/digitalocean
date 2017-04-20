extern crate digitalocean;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;
extern crate url;
extern crate url_serde;

mod utils;

use serde_json::Value;
use std::net::IpAddr;
use std::str::FromStr;

use digitalocean::api::{DropletAction, Droplet};
use digitalocean::request::Request;
use digitalocean::action::{Get, Create, List};

use utils::before;

#[test]
fn list_produces_correct_request() {
    before();

    let droplet_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/droplets/{}/actions", droplet_id);

    let req: Request<List, Vec<DropletAction>> = Droplet::get(droplet_id)
        .actions();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn backups_produces_correct_request() {
    before();

    let droplet_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/droplets/{}/actions", droplet_id);

    // Turn on.
    let req: Request<Create, DropletAction> = Droplet::get(droplet_id)
        .actions()
        .backups(true);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": "enable_backups",
    }));

    // Turn off.
    let req: Request<Create, DropletAction> = Droplet::get(droplet_id)
        .actions()
        .backups(false);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": "disable_backups",
    }));
}


#[test]
fn reboot_produces_correct_request() {
    before();

    let droplet_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/droplets/{}/actions", droplet_id);

    let req: Request<Create, DropletAction> = Droplet::get(droplet_id)
        .actions()
        .reboot();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": "reboot",
    }));
}

#[test]
fn power_cycle_produces_correct_request() {
    before();

    let droplet_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/droplets/{}/actions", droplet_id);

    let req: Request<Create, DropletAction> = Droplet::get(droplet_id)
        .actions()
        .power_cycle();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": "power_cycle",
    }));
}

#[test]
fn shutdown_produces_correct_request() {
    before();

    let droplet_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/droplets/{}/actions", droplet_id);

    let req: Request<Create, DropletAction> = Droplet::get(droplet_id)
        .actions()
        .shutdown();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": "shutdown",
    }));
}

#[test]
fn power_produces_correct_request() {
    before();

    let droplet_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/droplets/{}/actions", droplet_id);

    // Off
    let req: Request<Create, DropletAction> = Droplet::get(droplet_id)
        .actions()
        .power(false);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": "power_off",
    }));

    // On
    let req: Request<Create, DropletAction> = Droplet::get(droplet_id)
        .actions()
        .power(true);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": "power_on",
    }));
}

#[test]
fn restore_produces_correct_request() {
    before();

    let droplet_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/droplets/{}/actions", droplet_id);

    // As slug
    let image_id = "test";
    let req: Request<Create, DropletAction> = Droplet::get(droplet_id)
        .actions()
        .restore(image_id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": "restore",
        "image": image_id.to_string(),
    }));

    // As id
    let image_id = 456;
    let req: Request<Create, DropletAction> = Droplet::get(droplet_id)
        .actions()
        .restore(image_id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": "restore",
        "image": image_id.to_string(),
    }));
}

#[test]
fn password_reset_produces_correct_request() {
    before();

    let droplet_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/droplets/{}/actions", droplet_id);

    let req: Request<Create, DropletAction> = Droplet::get(droplet_id)
        .actions()
        .password_reset();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": "password_reset",
    }));
}

#[test]
fn resize_produces_correct_request() {
    before();

    let droplet_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/droplets/{}/actions", droplet_id);
    let (size, disk) = ("1gb", false);

    let req: Request<Create, DropletAction> = Droplet::get(droplet_id)
        .actions()
        .resize(size, disk);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": "resize",
        "size": size,
        "disk": disk,
    }));
}

#[test]
fn rebuild_produces_correct_request() {
    before();

    let droplet_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/droplets/{}/actions", droplet_id);
    let image_id = "test";

    let req: Request<Create, DropletAction> = Droplet::get(droplet_id)
        .actions()
        .rebuild(image_id.clone());
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": "rebuild",
        "image": image_id,
    }));
}

#[test]
fn rename_produces_correct_request() {
    before();

    let droplet_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/droplets/{}/actions", droplet_id);
    let new_name = "test";

    let req: Request<Create, DropletAction> = Droplet::get(droplet_id)
        .actions()
        .rename(new_name.clone());
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": "rename",
        "name": new_name,
    }));
}

#[test]
fn kernel_produces_correct_request() {
    before();

    let droplet_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/droplets/{}/actions", droplet_id);
    let kernel_id = 456;

    let req: Request<Create, DropletAction> = Droplet::get(droplet_id)
        .actions()
        .kernel(kernel_id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": "change_kernel",
        "kernel": kernel_id,
    }));
}

#[test]
fn enable_ipv6_kernel_produces_correct_request() {
    before();

    let droplet_id = 123;
    let correct_url = format!("https://api.digitalocean.com/v2/droplets/{}/actions", droplet_id);

    let req: Request<Create, DropletAction> = Droplet::get(droplet_id)
        .actions()
        .enable_ipv6();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": "enable_ipv6",
    }));
}

#[test]
fn enable_private_networking_produces_correct_request() {
    before();

    let droplet_id = 123;
    let correct_url =  format!("https://api.digitalocean.com/v2/droplets/{}/actions", droplet_id);

    let req: Request<Create, DropletAction> = Droplet::get(droplet_id)
        .actions()
        .enable_private_networking();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": "enable_private_networking",
    }));
}

#[test]
fn snapshot_produces_correct_request() {
    before();

    let droplet_id = 123;
    let correct_url = format!( "https://api.digitalocean.com/v2/droplets/{}/actions", droplet_id);
    let name = "blahblah";

    let req: Request<Create, DropletAction> = Droplet::get(droplet_id)
        .actions()
        .snapshot(name);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "type": "snapshot",
        "name": name,
    }));
}

#[test]
fn get_produces_correct_request() {
    before();

    let droplet_id = 123;
    let action_id = 456;
    let correct_url = format!("https://api.digitalocean.com/v2/droplets/{}/actions/{}", droplet_id, action_id);

    let req: Request<Get, DropletAction> = Droplet::get(droplet_id)
        .actions()
        .get(action_id);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}
