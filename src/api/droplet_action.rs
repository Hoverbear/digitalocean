use super::action::Action;
use super::droplet::Droplet;
use crate::method::{Create, Get, List};
use crate::request::{DropletActionRequest, DropletRequest};
use crate::STATIC_URL_ERROR;
use serde::Serialize;
use std::fmt::Display;

const DROPLET_ACTIONS_SEGMENT: &str = "actions";

impl DropletRequest<Get, Droplet> {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-actions-for-a-droplet)
    pub fn actions(mut self) -> DropletActionRequest<List, Vec<Action>> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.transmute()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#enable-backups)
    pub fn enable_backups(mut self) -> DropletActionRequest<Create, Action> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.set_body(json!({
            "type": "enable_backups",
        }));

        self.transmute()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#disable-backups)
    pub fn disable_backups(mut self) -> DropletActionRequest<Create, Action> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.set_body(json!({
            "type": "disable_backups",
        }));

        self.transmute()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#reboot-a-droplet)
    pub fn reboot(mut self) -> DropletActionRequest<Create, Action> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.set_body(json!({
            "type": "reboot",
        }));

        self.transmute()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#power-cycle-a-droplet)
    pub fn power_cycle(mut self) -> DropletActionRequest<Create, Action> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.set_body(json!({
            "type": "power_cycle",
        }));

        self.transmute()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#shutdown-a-droplet)
    pub fn shutdown(mut self) -> DropletActionRequest<Create, Action> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.set_body(json!({
            "type": "shutdown",
        }));

        self.transmute()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#power-off-a-droplet)
    pub fn power(mut self, val: bool) -> DropletActionRequest<Create, Action> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.set_body(json!({
            "type": if val { "power_on" } else { "power_off" },
        }));

        self.transmute()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#restore-a-droplet)
    pub fn restore<D>(mut self, image: D) -> DropletActionRequest<Create, Action>
    where
        D: Display,
    {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.set_body(json!({
            "type": "restore",
            "image": format!("{}", image),
        }));

        self.transmute()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#password-reset-a-droplet)
    pub fn password_reset(mut self) -> DropletActionRequest<Create, Action> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.set_body(json!({
            "type": "password_reset",
        }));

        self.transmute()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#resize-a-droplet)
    pub fn resize<S>(mut self, size: S, disk: bool) -> DropletActionRequest<Create, Action>
    where
        S: AsRef<str> + Serialize + Display,
    {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.set_body(json!({
            "type": "resize",
            "disk": disk,
            "size": size.as_ref(),
        }));

        self.transmute()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#resize-a-droplet)
    pub fn rebuild<S>(mut self, image: S) -> DropletActionRequest<Create, Action>
    where
        S: AsRef<str> + Serialize + Display,
    {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.set_body(json!({
            "type": "rebuild",
            "image": image.as_ref(),
        }));

        self.transmute()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#rename-a-droplet)
    pub fn rename<S>(mut self, name: S) -> DropletActionRequest<Create, Action>
    where
        S: AsRef<str> + Serialize + Display,
    {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.set_body(json!({
            "type": "rename",
            "name": name.as_ref(),
        }));

        self.transmute()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#change-the-kernel)
    pub fn kernel(mut self, kernel: usize) -> DropletActionRequest<Create, Action> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.set_body(json!({
            "type": "change_kernel",
            "kernel": kernel,
        }));

        self.transmute()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#enable-ipv6)
    pub fn enable_ipv6(mut self) -> DropletActionRequest<Create, Action> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.set_body(json!({
            "type": "enable_ipv6",
        }));

        self.transmute()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#enable-private-networking)
    pub fn enable_private_networking(mut self) -> DropletActionRequest<Create, Action> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.set_body(json!({
            "type": "enable_private_networking",
        }));

        self.transmute()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#snapshot-a-droplet)
    pub fn snapshot<S>(mut self, name: S) -> DropletActionRequest<Create, Action>
    where
        S: AsRef<str> + Serialize + Display,
    {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.set_body(json!({
            "type": "snapshot",
            "name": name.as_ref(),
        }));

        self.transmute()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-a-droplet-action)
    pub fn action(mut self, id: usize) -> DropletActionRequest<Get, Action> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT)
            .push(&id.to_string());

        self.transmute()
    }
}

// TODO: https://developers.digitalocean.com/documentation/v2/#acting-on-tagged-droplets
