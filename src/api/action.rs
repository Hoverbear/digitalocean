use super::{ApiLinks, ApiMeta};
use super::{HasPagination, HasResponse, HasValue};
use chrono::{DateTime, Utc};
use method::{Get, List};
use request::ActionRequest;
use request::Request;
use url::Url;
use {ROOT_URL, STATIC_URL_ERROR};

const ACTIONS_SEGMENT: &'static str = "actions";

/// Actions are records of events that have occurred on the resources in your
/// account. These can be things like rebooting a Droplet, or transferring an
/// image to a new region.
///
/// An action object is created every time one of these actions is initiated.
/// The action object contains information about the current status of the
/// action, start and complete timestamps, and the associated resource type
/// and ID.
///
/// Every action that creates an action object is available through this
/// endpoint. Completed actions are not removed from this list and are always
/// available for querying.
///
/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#actions)
#[derive(Deserialize, Serialize, Debug, Clone, Getters, Setters)]
pub struct Action {
    /// A unique identifier for each Droplet action event. This is used to
    /// reference a specific action that was requested.
    #[get = "pub"]
    id: usize,
    /// The current status of the action. The value of this attribute will be
    /// "in-progress", "completed", or "errored".
    #[get = "pub"]
    status: String,
    /// The type of action that the event is executing (reboot, power_off,
    /// etc.).
    #[get = "pub"]
    started_at: DateTime<Utc>,
    /// A time value given in ISO8601 combined date and time format that
    /// represents when the action was completed.
    #[get = "pub"]
    completed_at: Option<DateTime<Utc>>,
    /// A unique identifier for the resource that the action is associated
    /// with.
    #[get = "pub"]
    resource_id: usize,
    /// The type of resource that the action is associated with.
    #[get = "pub"]
    resource_type: String,
    // /// (deprecated) A slug representing the region where the action occurred.
    // #[get = "pub"]
    // #[deprecated(since = "0.0.1", note = "DigitalOcean has deprecated this.")]
    // region: Option<Region>,
    /// A slug representing the region where the action occurred.
    #[get = "pub"]
    region_slug: Option<String>,
}

impl Action {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-action)
    pub fn get(id: usize) -> ActionRequest<Get, Action> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(ACTIONS_SEGMENT)
            .push(&id.to_string());

        Request::new(url)
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-actions)
    pub fn list() -> ActionRequest<List, Vec<Action>> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(ACTIONS_SEGMENT);

        Request::new(url)
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ActionResponse {
    action: Action,
}

impl HasValue for ActionResponse {
    type Value = Action;
    fn value(self) -> Action {
        self.action
    }
}

impl HasResponse for Action {
    type Response = ActionResponse;
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ActionListResponse {
    actions: Vec<Action>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasResponse for Vec<Action> {
    type Response = ActionListResponse;
}

impl HasPagination for ActionListResponse {
    fn next_page(&self) -> Option<Url> {
        self.links.next()
    }
}

impl HasValue for ActionListResponse {
    type Value = Vec<Action>;
    fn value(self) -> Vec<Action> {
        self.actions
    }
}
