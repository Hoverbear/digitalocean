use request::Request;
use action::{Create, Get, List};
use super::FloatingIp;
use STATIC_URL_ERROR;
use url::Url;
use chrono::{DateTime, UTC};
use super::{ApiLinks, ApiMeta};
use super::{HasValue, HasPagination, HasResponse};

const FLOATING_IP_ACTIONS_SEGMENT: &'static str = "actions";

/// Floating IP actions are commands that can be given to a DigitalOcean 
/// Floating IP. These requests are made on the actions endpoint of a specific
/// Floating IP.
///
/// An action object is returned. These objects hold the current status of the
/// requested action.
///
/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#floating-ip-actions)
#[derive(Deserialize, Debug, Clone)]
pub struct FloatingIpAction {
    /// A unique numeric ID that can be used to identify and reference an
    /// action.
    pub id: usize,
    /// The current status of the action. This can be "in-progress", 
    /// "completed", or "errored".
    pub status: String,
    /// This is the type of action that the object represents. For example,
    /// this could be "assign_ip" to represent the state of a Floating IP 
    /// assign action.
    ///
    /// *Note:* Since `type` is a keyword in Rust `kind` is used instead.
    #[serde(rename = "type")]
    pub kind: String, // 'type' is reserved in Rust.
    /// A time value given in ISO8601 combined date and time format that
    /// represents when the action was initiated.
    pub started_at: DateTime<UTC>,
    /// A time value given in ISO8601 combined date and time format that
    /// represents when the action was completed.
    pub completed_at: DateTime<UTC>,
    /// A unique identifier for the resource that the action is associated 
    /// with.
    pub resource_id: usize,
    /// (deprecated) A slug representing the region where the action occurred.
    #[deprecated(since = "0.0.1", note="DigitalOcean has deprecated this.")]
    pub region: Option<String>,
    /// A slug representing the region where the action occurred.
    pub region_slug: Option<String>,
}

impl Request<Get, FloatingIp> {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-actions-for-a-floating-ip)
    pub fn actions(mut self) -> Request<List, Vec<FloatingIpAction>> {
        self.url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(FLOATING_IP_ACTIONS_SEGMENT);

        self.action()
            .value()
    }
}

impl Request<List, Vec<FloatingIpAction>> {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-floating-ip-action)
    pub fn get(mut self, id: usize) -> Request<Get, FloatingIpAction> {
        self.url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(&id.to_string());

        self.action()
            .value()
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#unassign-a-floating-ip)
    pub fn unassign(mut self) -> Request<Create, FloatingIpAction> {
        self.body = json!({
            "type": "unassign",
        });

        self.action()
            .value()
    }


    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#assign-a-floating-ip-to-a-droplet)
    pub fn assign(mut self, id: usize) -> Request<Create, FloatingIpAction> {
        self.body = json!({
            "type": "assign",
            "droplet_id": id,
        });

        self.action()
            .value()
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Debug, Clone)]
pub struct FloatingIpActionResponse {
    action: FloatingIpAction,
}

impl HasResponse for FloatingIpAction {
    type Response = FloatingIpActionResponse;
}

impl HasValue for FloatingIpActionResponse {
    type Value = FloatingIpAction;
    fn value(self) -> FloatingIpAction {
        self.action
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Debug, Clone)]
pub struct FloatingIpActionListResponse {
    actions: Vec<FloatingIpAction>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasResponse for Vec<FloatingIpAction> {
    type Response = FloatingIpActionListResponse;
}

impl HasPagination for FloatingIpActionListResponse {
    fn next_page(&self) -> Option<Url> {
        self.links.next()
    }
}

impl HasValue for FloatingIpActionListResponse {
    type Value = Vec<FloatingIpAction>;
    fn value(self) -> Vec<FloatingIpAction> {
        self.actions
    }
}
