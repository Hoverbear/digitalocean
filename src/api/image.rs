use serde::Serialize;
use std::fmt::Display;
use request::Request;
use method::{List, Get, Update, Delete};
use {ROOT_URL, STATIC_URL_ERROR};
use url::Url;
use chrono::{DateTime, UTC};
use super::{ApiLinks, ApiMeta};
use super::{HasValue, HasPagination, HasResponse};

const IMAGES_SEGMENT: &'static str = "images";

/// Images in DigitalOcean may refer to one of a few different kinds of objects.
///
/// An image may refer to a snapshot that has been taken of a Droplet instance.
/// It may also mean an image representing an automatic backup of a Droplet. 
/// The third category that it can represent is a public Linux distribution or
/// application image that is used as a base to create Droplets.
///
/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domains)
#[derive(Deserialize, Debug, Clone)]
pub struct Image {
    /// A unique number that can be used to identify and reference a specific
    /// image.
    pub id: usize,
    /// The display name that has been given to an image. This is what is shown
    /// in the control panel and is generally a descriptive title for the image
    /// in question.
    pub name: String,
    /// The kind of image, describing the duration of how long the image is 
    /// stored. This is either "snapshot" or "backup".
    ///
    /// *Note:* Since `type` is a keyword in Rust `kind` is used instead.
    #[serde(rename = "type")]
    pub kind: String, // 'type' is reserved in Rust.
    /// This attribute describes the base distribution used for this image.
    pub distribution: String,
    /// A uniquely identifying string that is associated with each of the
    /// DigitalOcean-provided public images. These can be used to reference
    /// a public image as an alternative to the numeric id.
    pub slug: Option<String>,
    /// This is a boolean value that indicates whether the image in question
    /// is public or not. An image that is public is available to all accounts.
    /// A non-public image is only accessible from your account.
    pub public: bool,
    /// This attribute is an array of the regions that the image is available
    /// in. The regions are represented by their identifying slug values.
    pub regions: Vec<String>,
    /// The minimum 'disk' required for a size to use this image.
    pub min_disk_size: usize,
    /// The size of the image in gigabytes.
    pub size_gigabytes: f32,
    /// A time value given in ISO8601 combined date and time format that 
    /// represents when the Image was created.
    pub created_at: DateTime<UTC>,
}

impl Image {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-images)
    pub fn list() -> Request<List, Vec<Image>> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(IMAGES_SEGMENT);

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-distribution-images)
    pub fn distributions() -> Request<List, Vec<Image>> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(IMAGES_SEGMENT);

        url.query_pairs_mut()
            .append_pair("type", "distribution");

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-application-images)
    pub fn applications() -> Request<List, Vec<Image>> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(IMAGES_SEGMENT);

        url.query_pairs_mut()
            .append_pair("type", "application");

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-a-user-s-images)
    pub fn user() -> Request<List, Vec<Image>> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(IMAGES_SEGMENT);

        url.query_pairs_mut()
            .append_pair("private", "true");

        Request::new(url)
    }

    /// `id` is either an `id` (numeric) or a `slug` (string).
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-image-by-id)
    pub fn get<S>(id: S) -> Request<Get, Image>
    where S: Display {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(IMAGES_SEGMENT)
            .push(&format!("{}", id));

        Request::new(url)
    }

    /// `id` is either an `id` (numeric) or a `slug` (string).
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#update-an-image)
    pub fn update<S>(id: S) -> Request<Update, Image>
    where S: Display {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(IMAGES_SEGMENT)
            .push(&format!("{}", id));

        Request::new(url)
    }

    /// `id` is either an `id` (numeric) or a `slug` (string).
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#delete-an-image)
    pub fn delete<S>(id: S) -> Request<Delete, ()>
    where S: Display {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(IMAGES_SEGMENT)
            .push(&format!("{}", id));

        Request::new(url)
    }
}

impl Request<Update, Image> {
    /// The new name that you would like to use for the image.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#images)
    pub fn name<S>(mut self, val: S) -> Request<Update, Image>
    where S: Display + Serialize {
        self.body["name"] = json!(val);
        self
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Debug, Clone)]
pub struct ImageResponse {
    image: Image,
}

impl HasResponse for Image {
    type Response = ImageResponse;
}

impl HasValue for ImageResponse {
    type Value = Image;
    fn value(self) -> Image {
        self.image
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Debug, Clone)]
pub struct ImageListResponse {
    images: Vec<Image>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasResponse for Vec<Image> {
    type Response = ImageListResponse;
}

impl HasPagination for ImageListResponse {
    fn next_page(&self) -> Option<Url> {
        self.links.next()
    }
}

impl HasValue for ImageListResponse {
    type Value = Vec<Image>;
    fn value(self) -> Vec<Image> {
        self.images
    }
}