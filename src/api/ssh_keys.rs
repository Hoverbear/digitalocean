//! Domain specific documentation.

use serde::Serialize;
use std::fmt::Display;
use request::Request;
use action::{List, Get, Create, Update, Delete};
use {ROOT_URL, STATIC_URL_ERROR};
use url::Url;
use values::SshKey;
use super::{ApiLinks, ApiMeta};
use super::{HasValue, HasPagination};

const ACCOUNT_SEGMENT: &'static str = "account";
const KEYS_SEGMENT: &'static str = "keys";

pub struct SshKeys;

impl SshKeys {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-key)
    pub fn create<N>(name: N, public_key: N) -> Request<Create, SshKey>
    where N: AsRef<str> + Serialize + Display {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(ACCOUNT_SEGMENT)
            .push(KEYS_SEGMENT);

        Request::new(url).body(json!({
            "name": name,
            "public_key": public_key,
        }))
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-keys)
    pub fn list() -> Request<List, Vec<SshKey>> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(ACCOUNT_SEGMENT)
            .push(KEYS_SEGMENT);

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-key)
    pub fn get<S>(id: S) -> Request<Get, SshKey> 
    where S: AsRef<str> + Serialize + Display {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(ACCOUNT_SEGMENT)
            .push(KEYS_SEGMENT)
            .push(id.as_ref());

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-key)
    pub fn update<S>(id: S) -> Request<Update, SshKey> 
    where S: AsRef<str> + Serialize + Display {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(ACCOUNT_SEGMENT)
            .push(KEYS_SEGMENT)
            .push(id.as_ref());

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#delete-a-domain)
    pub fn delete<S>(id: S) -> Request<Delete, ()> 
    where S: AsRef<str> + Serialize + Display {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(ACCOUNT_SEGMENT)
            .push(KEYS_SEGMENT)
            .push(id.as_ref());
        
        Request::new(url)
    }
}

impl Request<Update, SshKey> {
    /// The name to give the new SSH key in your account.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn name<S>(mut self, val: S) -> Self
    where S: AsRef<str> + Display + Serialize {
        self.body["name"] = json!(val);
        self
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct SshKeysListResponse {
    ssh_keys: Vec<SshKey>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasPagination for SshKeysListResponse {
    fn next_page(&self) -> Option<Url> {
        self.links.next()
    }
}

impl HasValue for SshKeysListResponse {
    type Value = Vec<SshKey>;
    fn value(self) -> Vec<SshKey> {
        self.ssh_keys
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct SshKeysResponse {
    ssh_key: SshKey,
}

impl HasValue for SshKeysResponse {
    type Value = SshKey;
    fn value(self) -> SshKey {
        self.ssh_key
    }
}