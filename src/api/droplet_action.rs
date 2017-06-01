use serde::Serialize;
use std::fmt::Display;
use request::Request;
use method::{List, Get, Create};
use STATIC_URL_ERROR;
use super::{Droplet, Action};

const DROPLET_ACTIONS_SEGMENT: &'static str = "actions";

impl Request<Get, Droplet> {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-actions-for-a-droplet)
    pub fn actions(mut self) -> Request<List, Vec<Action>> {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.method().value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#enable-backups)
    pub fn enable_backups(mut self) -> Request<Create, Action> {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.body = json!({
            "type": "enable_backups",
        });

        self.method().value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#disable-backups)
    pub fn disable_backups(mut self) -> Request<Create, Action> {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.body = json!({
            "type": "disable_backups",
        });

        self.method().value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#reboot-a-droplet)
    pub fn reboot(mut self) -> Request<Create, Action> {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.body = json!({
            "type": "reboot",
        });

        self.method().value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#power-cycle-a-droplet)
    pub fn power_cycle(mut self) -> Request<Create, Action> {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.body = json!({
            "type": "power_cycle",
        });

        self.method().value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#shutdown-a-droplet)
    pub fn shutdown(mut self) -> Request<Create, Action> {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.body = json!({
            "type": "shutdown",
        });

        self.method().value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#power-off-a-droplet)
    pub fn power(mut self, val: bool) -> Request<Create, Action> {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.body = json!({
            "type": if val { "power_on" } else { "power_off" },
        });

        self.method().value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#restore-a-droplet)
    pub fn restore<D>(mut self, image: D) -> Request<Create, Action>
        where D: Display
    {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.body = json!({
            "type": "restore",
            "image": format!("{}", image),
        });

        self.method().value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#password-reset-a-droplet)
    pub fn password_reset(mut self) -> Request<Create, Action> {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.body = json!({
            "type": "password_reset",
        });

        self.method().value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#resize-a-droplet)
    pub fn resize<S>(mut self, size: S, disk: bool) -> Request<Create, Action>
        where S: AsRef<str> + Serialize + Display
    {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.body = json!({
            "type": "resize",
            "disk": disk,
            "size": size.as_ref(),
        });

        self.method().value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#resize-a-droplet)
    pub fn rebuild<S>(mut self, image: S) -> Request<Create, Action>
        where S: AsRef<str> + Serialize + Display
    {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.body = json!({
            "type": "rebuild",
            "image": image.as_ref(),
        });

        self.method().value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#rename-a-droplet)
    pub fn rename<S>(mut self, name: S) -> Request<Create, Action>
        where S: AsRef<str> + Serialize + Display
    {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.body = json!({
            "type": "rename",
            "name": name.as_ref(),
        });

        self.method().value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#change-the-kernel)
    pub fn kernel(mut self, kernel: usize) -> Request<Create, Action> {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.body = json!({
            "type": "change_kernel",
            "kernel": kernel,
        });

        self.method().value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#enable-ipv6)
    pub fn enable_ipv6(mut self) -> Request<Create, Action> {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.body = json!({
            "type": "enable_ipv6",
        });

        self.method().value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#enable-private-networking)
    pub fn enable_private_networking(mut self) -> Request<Create, Action> {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.body = json!({
            "type": "enable_private_networking",
        });

        self.method().value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#snapshot-a-droplet)
    pub fn snapshot<S>(mut self, name: S) -> Request<Create, Action>
        where S: AsRef<str> + Serialize + Display
    {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.body = json!({
            "type": "snapshot",
            "name": name.as_ref(),
        });

        self.method().value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-a-droplet-action)
    pub fn action(mut self, id: usize) -> Request<Get, Action> {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT)
            .push(&id.to_string());

        self.method().value()
    }
}

// TODO: https://developers.digitalocean.com/documentation/v2/#acting-on-tagged-droplets
