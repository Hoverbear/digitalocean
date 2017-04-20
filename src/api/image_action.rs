use serde::Serialize;
use std::fmt::Display;
use request::Request;
use action::{List, Get, Create};
use url::Url;
use STATIC_URL_ERROR;
use super::Image;
use super::{ApiLinks, ApiMeta};
use super::{HasValue, HasPagination, HasResponse};

const IMAGE_ACTIONS_SEGMENT: &'static str = "actions";

/// Image actions are commands that can be given to a DigitalOcean image.
/// In general, these requests are made on the actions endpoint of a specific image.
///
/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domains)
#[derive(Deserialize, Debug, Clone)]
pub struct ImageAction {
    /// A unique numeric ID that can be used to identify and reference an image
    /// action.
    pub id: usize,
    /// The current status of the image action. This will be either
    /// "in-progress", "completed", or "errored".
    pub status: String,
    /// This is the type of the image action that the JSON object represents.
    /// For example, this could be "transfer" to represent the state of an 
    /// image transfer action.
    ///
    /// *Note:* Since `type` is a keyword in Rust `kind` is used instead.
    #[serde(rename = "type")]
    pub kind: String, // 'type' is reserved in Rust.
    /// A time value given in ISO8601 combined date and time format that 
    /// represents when the action was initiated.
    pub started_at: String,
    /// A time value given in ISO8601 combined date and time format that
    /// represents when the action was completed.
    pub completed_at: String,
    /// A unique identifier for the resource that the action is associated with.
    pub resource_id: usize,
    /// The type of resource that the action is associated with.
    pub resource_type: String,
    /// (deprecated) A slug representing the region where the action occurred.
    pub region: Option<String>,
    /// A slug representing the region where the action occurred.
    pub region_slug: Option<String>,

}

impl Request<Get, Image> {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-actions-for-an-image)
    pub fn actions(mut self) -> Request<List, Vec<ImageAction>> {
        self.url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(IMAGE_ACTIONS_SEGMENT);

        self.action()
            .value()
    }
}

impl Request<List, Vec<ImageAction>> {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#transfer-an-image)
    pub fn transfer<S>(mut self, region: S) -> Request<Create, ImageAction>
    where S: AsRef<str> + Display + Serialize {
        self.body = json!({
            "type": "transfer",
            "region": region,
        });

        self.action()
            .value()
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#convert-an-image-to-a-snapshot)
    pub fn convert(mut self) -> Request<Create, ImageAction> {
        self.body = json!({
            "type": "convert",
        });

        self.action()
            .value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-image-action)
    pub fn get(mut self, id: usize) -> Request<Get, ImageAction> {
        self.url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(&id.to_string());
        
        self.action()
            .value()
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Debug, Clone)]
pub struct ImageActionResponse {
    action: ImageAction,
}


impl HasValue for ImageActionResponse {
    type Value = ImageAction;
    fn value(self) -> ImageAction {
        self.action
    }
}

impl HasResponse for ImageAction {
    type Response = ImageActionResponse;
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Debug, Clone)]
pub struct ImageActionListResponse {
    actions: Vec<ImageAction>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasResponse for Vec<ImageAction> {
    type Response = ImageActionListResponse;
}

impl HasPagination for ImageActionListResponse {
    fn next_page(&self) -> Option<Url> {
        self.links.next()
    }
}

impl HasValue for ImageActionListResponse {
    type Value = Vec<ImageAction>;
    fn value(self) -> Vec<ImageAction> {
        self.actions
    }
}
