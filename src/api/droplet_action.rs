use serde::Serialize;
use std::fmt::Display;
use request::Request;
use action::{List, Get, Create};
use STATIC_URL_ERROR;
use url::Url;
use chrono::{DateTime, UTC};
use super::Droplet;
use super::{ApiLinks, ApiMeta};
use super::{HasValue, HasPagination, HasResponse};

const DROPLET_ACTIONS_SEGMENT: &'static str = "actions";

/// Droplet actions are tasks that can be executed on a Droplet. These can be
/// things like rebooting, resizing, snapshotting, etc.
///
/// Droplet action requests are generally targeted at one of the "actions" 
/// endpoints for a specific Droplet. The specific actions are usually 
/// initiated by sending a POST request with the action and arguments as 
/// parameters.
///
/// Droplet action requests create a Droplet actions object, which can be used
/// to get information about the status of an action. Creating a Droplet action
/// is asynchronous: the HTTP call will return the action object before the
/// action has finished processing on the Droplet. The current status of an
/// action can be retrieved from either the Droplet actions endpoint or the
/// global actions endpoint. If a Droplet action is uncompleted it may block
/// the creation of a subsequent action for that Droplet, the locked attribute
/// of the Droplet will be true and attempts to create a Droplet action will
/// fail with a status of 422.
///
/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#droplet-actions)
#[derive(Deserialize, Debug, Clone)]
pub struct DropletAction {
    /// A unique identifier for each Droplet action event. This is used to
    /// reference a specific action that was requested.
    pub id: usize,
    /// The current status of the action. The value of this attribute will be
    /// "in-progress", "completed", or "errored".
    pub status: String,
    /// The type of action that the event is executing (reboot, power_off, 
    /// etc.).
    pub started_at: DateTime<UTC>,
    /// A time value given in ISO8601 combined date and time format that
    /// represents when the action was completed.
    pub completed_at: DateTime<UTC>,
    /// A unique identifier for the resource that the action is associated 
    /// with.
    pub resource_id: usize,
    /// The type of resource that the action is associated with.
    pub resource_type: String,
    /// (deprecated) A slug representing the region where the action occurred.
    #[deprecated(since = "0.0.1", note="DigitalOcean has deprecated this.")]
    pub region: Option<String>,
    /// A slug representing the region where the action occurred.
    pub region_slug: Option<String>,
}

impl Request<Get, Droplet> {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-actions-for-a-droplet)
    pub fn actions(mut self) -> Request<List, Vec<DropletAction>> {
        self.url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLET_ACTIONS_SEGMENT);

        self.action()
            .value()
    }
}


impl Request<List, Vec<DropletAction>> {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#enable-backups)
    pub fn backups(mut self, val: bool) -> Request<Create, DropletAction> {
        self.body = json!({
            "type": if val { "enable_backups" } else { "disable_backups" },
        });

        self.action()
            .value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#reboot-a-droplet)
    pub fn reboot(mut self) -> Request<Create, DropletAction> {
        self.body = json!({
            "type": "reboot",
        });

        self.action()
            .value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#power-cycle-a-droplet)
    pub fn power_cycle(mut self) -> Request<Create, DropletAction> {
        self.body = json!({
            "type": "power_cycle",
        });

        self.action()
            .value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#shutdown-a-droplet)
    pub fn shutdown(mut self) -> Request<Create, DropletAction> {
        self.body = json!({
            "type": "shutdown",
        });

        self.action()
            .value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#power-off-a-droplet)
    pub fn power(mut self, val: bool) -> Request<Create, DropletAction> {
        self.body = json!({
            "type": if val { "power_on" } else { "power_off" },
        });

        self.action()
            .value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#restore-a-droplet)
    pub fn restore<D>(mut self, image: D) -> Request<Create, DropletAction>
    where D: Display {
        self.body = json!({
            "type": "restore",
            "image": format!("{}", image),
        });

        self.action()
            .value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#password-reset-a-droplet)
    pub fn password_reset(mut self) -> Request<Create, DropletAction> {
        self.body = json!({
            "type": "password_reset",
        });

        self.action()
            .value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#resize-a-droplet)
    pub fn resize<S>(mut self, size: S, disk: bool) -> Request<Create, DropletAction>
    where S: AsRef<str> + Serialize + Display {
        self.body = json!({
            "type": "resize",
            "disk": disk,
            "size": size.as_ref(),
        });

        self.action()
            .value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#resize-a-droplet)
    pub fn rebuild<S>(mut self, image: S) -> Request<Create, DropletAction>
    where S: AsRef<str> + Serialize + Display {
        self.body = json!({
            "type": "rebuild",
            "image": image.as_ref(),
        });

        self.action()
            .value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#rename-a-droplet)
    pub fn rename<S>(mut self, name: S) -> Request<Create, DropletAction>
    where S: AsRef<str> + Serialize + Display {
        self.body = json!({
            "type": "rename",
            "name": name.as_ref(),
        });

        self.action()
            .value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#change-the-kernel)
    pub fn kernel(mut self, kernel: usize) -> Request<Create, DropletAction> {
        self.body = json!({
            "type": "change_kernel",
            "kernel": kernel,
        });

        self.action()
            .value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#enable-ipv6)
    pub fn enable_ipv6(mut self) -> Request<Create, DropletAction> {
        self.body = json!({
            "type": "enable_ipv6",
        });

        self.action()
            .value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#enable-private-networking)
    pub fn enable_private_networking(mut self) -> Request<Create, DropletAction> {
        self.body = json!({
            "type": "enable_private_networking",
        });

        self.action()
            .value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#snapshot-a-droplet)
    pub fn snapshot<S>(mut self, name: S) -> Request<Create, DropletAction>
    where S: AsRef<str> + Serialize + Display {
        self.body = json!({
            "type": "snapshot",
            "name": name.as_ref(),
        });

        self.action()
            .value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-a-droplet-action)
    pub fn get(mut self, id: usize) -> Request<Get, DropletAction> {
        self.url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(&id.to_string());

        self.action()
            .value()
    }
}

// TODO: https://developers.digitalocean.com/documentation/v2/#acting-on-tagged-droplets

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Debug, Clone)]
pub struct DropletActionResponse {
    action: DropletAction,
}

impl HasResponse for DropletAction {
    type Response = DropletActionResponse;
}

impl HasValue for DropletActionResponse {
    type Value = DropletAction;
    fn value(self) -> DropletAction {
        self.action
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Debug, Clone)]
pub struct DropletActionListResponse {
    actions: Vec<DropletAction>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasResponse for Vec<DropletAction> {
    type Response = DropletActionListResponse;
}

impl HasPagination for DropletActionListResponse {
    fn next_page(&self) -> Option<Url> {
        self.links.next()
    }
}

impl HasValue for DropletActionListResponse {
    type Value = Vec<DropletAction>;
    fn value(self) -> Vec<DropletAction> {
        self.actions
    }
}
