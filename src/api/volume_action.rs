use serde::Serialize;
use std::fmt::Display;
use request::Request;
use method::{List, Get, Create};
use {ROOT_URL, STATIC_URL_ERROR};
use super::{Action, Volume};

const VOLUMES_SEGMENT: &'static str = "volumes";
const VOLUME_ACTIONS_SEGMENT: &'static str = "actions";

impl Volume {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#attach-a-block-storage-volume-to-a-droplet-by-name)
    pub fn attach<S>(volume_name: S, droplet: usize) -> Request<Create, Action>
    where S: AsRef<str> + Serialize + Display {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(VOLUMES_SEGMENT);

        Request::new(url).body(json!({
            "type": "attach",
            "volume_name": volume_name,
            "droplet_id": droplet,
        }))
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#remove-a-block-storage-volume-from-a-droplet-by-name)
    pub fn detach<S>(volume_name: S, droplet: usize) -> Request<Create, Action>
    where S: AsRef<str> + Serialize + Display {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(VOLUMES_SEGMENT);

        Request::new(url).body(json!({
            "type": "detach",
            "volume_name": volume_name,
            "droplet_id": droplet,
        }))
    }
}

impl Request<Get, Volume> {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#attach-a-block-storage-volume-to-a-droplet)
    pub fn attach(mut self, droplet: usize) -> Request<Create, Action> {
        self.url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(VOLUME_ACTIONS_SEGMENT);

        self.body = json!({
            "type": "attach",
            "droplet_id": droplet,
        });

        self.method()
            .value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#remove-a-block-storage-volume-from-a-droplet)
    pub fn detach(mut self, droplet: usize) -> Request<Create, Action> {
        self.url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(VOLUME_ACTIONS_SEGMENT);

        self.body = json!({
            "type": "detach",
            "droplet_id": droplet,
        });

        self.method()
            .value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#resize-a-volume)
    pub fn resize(mut self, size: usize) -> Request<Create, Action> {
        self.url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(VOLUME_ACTIONS_SEGMENT);

        self.body = json!({
            "type": "resize",
            "size_gigabytes": size,
        });

        self.method()
            .value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-actions-for-a-volume)
    pub fn actions(mut self) -> Request<List, Vec<Action>> {
        self.url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(VOLUME_ACTIONS_SEGMENT);

        self.method()
            .value()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-volume-action)
    pub fn action(mut self, id: usize) -> Request<Get, Action> {
        self.url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(VOLUME_ACTIONS_SEGMENT)
            .push(&id.to_string());
        
        self.method()
            .value()
    }
}
