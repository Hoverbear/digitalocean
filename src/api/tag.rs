use serde::Serialize;
use std::fmt::Display;
use request::Request;
use method::{Get, Create, List, Delete};
use {ROOT_URL, STATIC_URL_ERROR};
use url::Url;
use serde_json::Value;
use super::{ApiLinks, ApiMeta};
use super::{HasValue, HasPagination, HasResponse};

const TAG_SEGMENT: &'static str = "tags";
const RESOURCES_SEGMENT: &'static str = "resources";

/// A Tag is a label that can be applied to a resource (currently only
/// Droplets) in order to better organize or facilitate the lookups and actions
///  on it.
///
/// Tags have two attributes: a user defined name attribute and an embedded
/// resources attribute with information about resources that have been tagged.
///
/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#tags)
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tag {
    /// Tags may contain letters, numbers, colons, dashes, and underscores.
    /// There is a limit of 255 characters per tag.
    pub name: String,
    /// An embedded object containing key value pairs of resource type and
    /// resource statistics.
    pub resources: Value,
}

impl Tag {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-tag)
    pub fn create<S>(name: S) -> Request<Create, Tag>
        where S: AsRef<str> + Serialize + Display
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(TAG_SEGMENT);

        Request::new(url).body(json!({
            "name": name,
        }))
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-a-tag)
    pub fn get<S>(name: S) -> Request<Get, Tag>
        where S: AsRef<str> + Serialize + Display
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(TAG_SEGMENT)
            .push(name.as_ref());

        Request::new(url)
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-a-tag)
    pub fn list() -> Request<List, Tag> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(TAG_SEGMENT);

        Request::new(url)
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#delete-a-tag)
    pub fn delete<S>(name: S) -> Request<Delete, ()>
        where S: AsRef<str> + Serialize + Display
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(TAG_SEGMENT)
            .push(name.as_ref());

        Request::new(url)
    }
}

impl Request<Get, Tag> {
    /// Accepts tuples matching `(id, type)`. Currently the only `type` is `"droplet"`.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#tag-a-resource)
    pub fn add_resources<S>(mut self, resources: Vec<(S, S)>) -> Request<Create, ()>
        where S: AsRef<str> + Serialize + Display
    {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(RESOURCES_SEGMENT);

        let resources = resources
            .into_iter()
            .map(|(id, kind)| {
                     json!({
            "resource_id": id,
            "resource_type": kind,
        })
                 })
            .collect::<Vec<_>>();

        self.body = json!({
            "resources": resources,
        });

        self.method().value()
    }
    /// Accepts tuples matching `(id, type)`. Currently the only `type` is `"droplet"`.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#untag-a-resource)
    pub fn remove_resources<S>(mut self, resources: Vec<(S, S)>) -> Request<Delete, ()>
        where S: AsRef<str> + Serialize + Display
    {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(RESOURCES_SEGMENT);

        let resources = resources
            .into_iter()
            .map(|(id, kind)| {
                     json!({
            "resource_id": id,
            "resource_type": kind,
        })
                 })
            .collect::<Vec<_>>();

        self.body = json!({
            "resources": resources,
        });

        self.method().value()
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TagResponse {
    tag: Tag,
}


impl HasValue for TagResponse {
    type Value = Tag;
    fn value(self) -> Tag {
        self.tag
    }
}

impl HasResponse for Tag {
    type Response = TagResponse;
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TagListResponse {
    tags: Vec<Tag>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasResponse for Vec<Tag> {
    type Response = TagListResponse;
}

impl HasPagination for TagListResponse {
    fn next_page(&self) -> Option<Url> {
        self.links.next()
    }
}

impl HasValue for TagListResponse {
    type Value = Vec<Tag>;
    fn value(self) -> Vec<Tag> {
        self.tags
    }
}
