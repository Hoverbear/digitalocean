//! Domain specific documentation.

use serde::Serialize;
use std::fmt::Display;
use request::Request;
use action::{List, Get, Create, Update, Delete};
use {ROOT_URL, STATIC_URL_ERROR};
use url::Url;
use super::{ApiLinks, ApiMeta};
use super::{HasValue, HasPagination, HasResponse};

const ACCOUNT_SEGMENT: &'static str = "account";
const KEYS_SEGMENT: &'static str = "keys";

/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#ssh-keys)
#[derive(Deserialize, Debug, Clone)]
pub struct SshKey {
    /// While this is technically an integer, Get/Update/Delete calls to the
    /// API work on either `id` or `fingerprint`, so we keep them the same type.
    pub id: String,
    pub fingerprint: String,
    pub public_key: String,
    pub name: String
}

impl SshKey {
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



/// Response type returned from Digital Ocean.
#[derive(Deserialize, Debug, Clone)]
pub struct SshKeysListResponse {
    ssh_keys: Vec<SshKey>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasResponse for Vec<SshKey> {
    type Response = SshKeysListResponse;
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

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Debug, Clone)]
pub struct SshKeysResponse {
    ssh_key: SshKey,
}

impl HasResponse for SshKey {
    type Response = SshKeysResponse;
}

impl HasValue for SshKeysResponse {
    type Value = SshKey;
    fn value(self) -> SshKey {
        self.ssh_key
    }
}