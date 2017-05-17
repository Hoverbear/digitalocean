//! API specific documentation.

mod account;
mod action;
mod certificate;
mod domain_record;
mod domain;
mod droplet_action;
mod droplet;
mod load_balancer;
mod floating_ip_action;
mod floating_ip;
mod image_action;
mod image;
mod region;
mod size;
mod snapshot;
mod ssh_key;
mod volume_action;
mod volume;
mod tag;

use url::Url;
use serde::de::DeserializeOwned;
use url_serde;

pub use self::account::Account;
pub use self::action::Action;
pub use self::certificate::Certificate;
pub use self::domain_record::DomainRecord;
pub use self::domain::Domain;
pub use self::load_balancer::{LoadBalancer, load_balancer_fields};
pub use self::droplet::{Droplet, droplet_fields};
pub use self::floating_ip::FloatingIp;
pub use self::image::Image;
pub use self::region::Region;
pub use self::size::Size;
pub use self::snapshot::Snapshot;
pub use self::ssh_key::SshKey;
pub use self::volume::Volume;
pub use self::tag::Tag;

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
    #[serde(with = "url_serde")]
    prev: Option<Url>,
    #[serde(with = "url_serde")]
    first: Option<Url>,
    #[serde(with = "url_serde")]
    next: Option<Url>,
    #[serde(with = "url_serde")]
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
    fn value(self) -> Self::Value { () }
}

pub trait HasResponse: DeserializeOwned + Clone {
    type Response: DeserializeOwned + Clone + HasValue<Value=Self>;
}

impl HasResponse for () {
    type Response = ();
}