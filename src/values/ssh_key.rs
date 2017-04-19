//! Values SSH Key documentation.

use super::HasResponse;
use api::ssh_keys::{SshKeysResponse, SshKeysListResponse};

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

impl HasResponse for SshKey {
    type Response = SshKeysResponse;
}

impl HasResponse for Vec<SshKey> {
    type Response = SshKeysListResponse;
}


