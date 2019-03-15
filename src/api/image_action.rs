use super::action::Action;
use super::image::Image;
use crate::method::{Create, Get, List};
use crate::request::{ImageActionRequest, ImageRequest};
use crate::STATIC_URL_ERROR;
use serde::Serialize;
use std::fmt::Display;

const IMAGE_ACTIONS_SEGMENT: &str = "actions";

impl ImageRequest<Get, Image> {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-actions-for-an-image)
    pub fn actions(mut self) -> ImageActionRequest<List, Vec<Action>> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(IMAGE_ACTIONS_SEGMENT);

        self.transmute()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#transfer-an-image)
    pub fn transfer<S>(mut self, region: S) -> ImageActionRequest<Create, Action>
    where
        S: AsRef<str> + Display + Serialize,
    {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(IMAGE_ACTIONS_SEGMENT);

        self.set_body(json!({
            "type": "transfer",
            "region": region,
        }));

        self.transmute()
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#convert-an-image-to-a-snapshot)
    pub fn convert(mut self) -> ImageActionRequest<Create, Action> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(IMAGE_ACTIONS_SEGMENT);

        self.set_body(json!({
            "type": "convert",
        }));

        self.transmute()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-image-action)
    pub fn action(mut self, id: usize) -> ImageActionRequest<Get, Action> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(IMAGE_ACTIONS_SEGMENT)
            .push(&id.to_string());

        self.transmute()
    }
}
