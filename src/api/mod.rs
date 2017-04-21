//! API specific documentation.

mod account;
mod action;
mod certificate;
mod domain_record;
mod domain;
mod droplet_action;
mod droplet;
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

use url::Url;
use serde::Deserialize;
use url_serde::SerdeUrl;

pub use self::account::Account;
pub use self::action::Action;
pub use self::certificate::Certificate;
pub use self::domain_record::DomainRecord;
pub use self::domain::Domain;
pub use self::droplet::{Droplet, droplet_fields};
pub use self::floating_ip::FloatingIp;
pub use self::image::Image;
pub use self::region::Region;
pub use self::size::Size;
pub use self::snapshot::Snapshot;
pub use self::ssh_key::SshKey;
pub use self::volume::Volume;

// Defined in https://developers.digitalocean.com/documentation/v2/#links
pub const MAX_PER_PAGE: usize = 200;

#[derive(Deserialize, Debug, Clone)]
struct ApiLinks {
    pages: Option<ApiPages>,
}

impl ApiLinks {
    fn next(&self) -> Option<Url> {
        match self.pages {
            Some(ref pages) => match pages.next {
                Some(ref v) => Some(v.clone().into_inner()),
                None => None,
            },
            None => None,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct ApiPages {
    prev: Option<SerdeUrl>,
    first: Option<SerdeUrl>,
    next: Option<SerdeUrl>,
    last: Option<SerdeUrl>,
}

#[derive(Deserialize, Debug, Clone)]
struct ApiMeta {
    total: usize,
}

pub trait HasPagination {
    fn next_page(&self) -> Option<Url>;
}

pub trait HasValue {
    type Value: Deserialize;
    fn value(self) -> Self::Value;
}

impl HasValue for () {
    type Value = ();
    fn value(self) -> Self::Value { () }
}

pub trait HasResponse: Deserialize + Deserialize + Clone {
    type Response: Deserialize + Clone + HasValue<Value=Self>;
}

impl HasResponse for () {
    type Response = ();
}