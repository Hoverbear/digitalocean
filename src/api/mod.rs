//! API specific documentation.

mod account;
mod action;
mod certificate;
mod custom_image;
mod domain;
mod domain_record;
mod droplet;
mod droplet_action;
mod floating_ip;
mod floating_ip_action;
mod image;
mod image_action;
mod load_balancer;
mod region;
mod size;
mod snapshot;
mod ssh_key;
mod tag;
mod volume;
mod volume_action;

use serde::de::DeserializeOwned;
use url::Url;
use url_serde;

pub use self::account::Account;
pub use self::action::Action;
pub use self::certificate::Certificate;
pub use self::custom_image::CustomImage;
pub use self::domain::Domain;
pub use self::domain_record::DomainRecord;
pub use self::droplet::{droplet_fields, Droplet};
pub use self::floating_ip::FloatingIp;
pub use self::image::Image;
pub use self::load_balancer::{load_balancer_fields, LoadBalancer};
pub use self::region::Region;
pub use self::size::Size;
pub use self::snapshot::Snapshot;
pub use self::ssh_key::SshKey;
pub use self::tag::Tag;
pub use self::volume::Volume;

// Defined in https://developers.digitalocean.com/documentation/v2/#links
pub const MAX_PER_PAGE: usize = 200;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ApiLinks {
    pages: Option<ApiPages>,
}

impl ApiLinks {
    fn next(&self) -> Option<Url> {
        match self.pages {
            Some(ref pages) => match pages.next {
                Some(ref v) => Some(v.clone()),
                None => None,
            },
            None => None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ApiPages {
    #[serde(with = "url_serde", default)]
    prev: Option<Url>,
    #[serde(with = "url_serde", default)]
    first: Option<Url>,
    #[serde(with = "url_serde", default)]
    next: Option<Url>,
    #[serde(with = "url_serde", default)]
    last: Option<Url>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ApiMeta {
    total: usize,
}

pub trait HasPagination {
    fn next_page(&self) -> Option<Url>;
}

pub trait HasValue {
    type Value: DeserializeOwned;
    fn value(self) -> Self::Value;
}

impl HasValue for () {
    type Value = ();
    fn value(self) -> Self::Value {
        ()
    }
}

pub trait HasResponse: DeserializeOwned + Clone {
    type Response: DeserializeOwned + Clone + HasValue<Value = Self>;
}

impl HasResponse for () {
    type Response = ();
}
