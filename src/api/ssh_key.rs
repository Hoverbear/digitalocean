use super::{ApiLinks, ApiMeta};
use super::{HasPagination, HasResponse, HasValue};
use method::{Create, Delete, Get, List, Update};
use request::Request;
use request::SshKeyRequest;
use serde::Serialize;
use std::fmt::Display;
use url::Url;
use {ROOT_URL, STATIC_URL_ERROR};

const ACCOUNT_SEGMENT: &'static str = "account";
const KEYS_SEGMENT: &'static str = "keys";

/// DigitalOcean allows you to add SSH public keys to the interface so that you
/// can embed your public key into a Droplet at the time of creation. Only the
/// public key is required to take advantage of this functionality.
///
/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#ssh-keys)
#[derive(Deserialize, Serialize, Debug, Clone, Getters, Setters)]
pub struct SshKey {
    /// This is a unique identification number for the key. This can be used
    /// to reference a specific SSH key when you wish to embed a key into a
    /// Droplet.
    ///
    /// *Note:* This is a `String` to allow for `id` and `fingerprint` to be
    /// used in `Get`, `Update`, and `Delete` calls like the API describes.
    #[get = "pub"]
    id: usize,
    /// This attribute contains the fingerprint value that is generated from
    /// the public key. This is a unique identifier that will differentiate
    /// it from other keys using a format that SSH recognizes.
    #[get = "pub"]
    fingerprint: String,
    /// This attribute contains the entire public key string that was uploaded.
    /// This is what is embedded into the root user's authorized_keys file if
    /// you choose to include this SSH key during Droplet creation.
    #[get = "pub"]
    public_key: String,
    /// This is the human-readable display name for the given SSH key. This
    /// is used to easily identify the SSH keys when they are displayed.
    #[get = "pub"]
    name: String,
}

impl SshKey {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-key)
    pub fn create<N>(name: N, public_key: N) -> SshKeyRequest<Create, SshKey>
    where
        N: AsRef<str> + Serialize + Display,
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(ACCOUNT_SEGMENT)
            .push(KEYS_SEGMENT);

        let mut req = Request::new(url);
        req.set_body(json!({
            "name": name,
            "public_key": public_key,
        }));
        req
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-keys)
    pub fn list() -> SshKeyRequest<List, Vec<SshKey>> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(ACCOUNT_SEGMENT)
            .push(KEYS_SEGMENT);

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-key)
    pub fn get<S>(id: S) -> SshKeyRequest<Get, SshKey>
    where
        S: Serialize + Display,
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(ACCOUNT_SEGMENT)
            .push(KEYS_SEGMENT)
            .push(&format!("{}", id));

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-key)
    pub fn update<S>(id: S) -> SshKeyRequest<Update, SshKey>
    where
        S: Serialize + Display,
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(ACCOUNT_SEGMENT)
            .push(KEYS_SEGMENT)
            .push(&format!("{}", id));

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#delete-a-domain)
    pub fn delete<S>(id: S) -> SshKeyRequest<Delete, ()>
    where
        S: Serialize + Display,
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(ACCOUNT_SEGMENT)
            .push(KEYS_SEGMENT)
            .push(&format!("{}", id));

        Request::new(url)
    }
}

impl SshKeyRequest<Update, SshKey> {
    /// The name to give the new SSH key in your account.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn name<S>(mut self, val: S) -> Self
    where
        S: AsRef<str> + Display + Serialize,
    {
        self.body_mut()["name"] = json!(val);
        self
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SshKeyListResponse {
    ssh_keys: Vec<SshKey>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasResponse for Vec<SshKey> {
    type Response = SshKeyListResponse;
}

impl HasPagination for SshKeyListResponse {
    fn next_page(&self) -> Option<Url> {
        self.links.next()
    }
}

impl HasValue for SshKeyListResponse {
    type Value = Vec<SshKey>;
    fn value(self) -> Vec<SshKey> {
        self.ssh_keys
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SshKeyResponse {
    ssh_key: SshKey,
}

impl HasResponse for SshKey {
    type Response = SshKeyResponse;
}

impl HasValue for SshKeyResponse {
    type Value = SshKey;
    fn value(self) -> SshKey {
        self.ssh_key
    }
}
