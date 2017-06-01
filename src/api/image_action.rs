use serde::Serialize;
use std::fmt::Display;
use request::Request;
use method::{List, Get, Create};
use STATIC_URL_ERROR;
use super::{Image, Action};

const IMAGE_ACTIONS_SEGMENT: &'static str = "actions";

impl Request<Get, Image> {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-actions-for-an-image)
    pub fn actions(mut self) -> Request<List, Vec<Action>> {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(IMAGE_ACTIONS_SEGMENT);

        self.method().value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#transfer-an-image)
    pub fn transfer<S>(mut self, region: S) -> Request<Create, Action>
        where S: AsRef<str> + Display + Serialize
    {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(IMAGE_ACTIONS_SEGMENT);

        self.body = json!({
            "type": "transfer",
            "region": region,
        });

        self.method().value()
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#convert-an-image-to-a-snapshot)
    pub fn convert(mut self) -> Request<Create, Action> {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(IMAGE_ACTIONS_SEGMENT);

        self.body = json!({
            "type": "convert",
        });

        self.method().value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-image-action)
    pub fn action(mut self, id: usize) -> Request<Get, Action> {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(IMAGE_ACTIONS_SEGMENT)
            .push(&id.to_string());

        self.method().value()
    }
}
