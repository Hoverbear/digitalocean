//! API specific documentation.

mod domain;
mod domain_record;
mod ssh_key;

use url::Url;
use serde::Deserialize;
use url_serde::SerdeUrl;

pub use self::domain::Domain;
pub use self::ssh_key::SshKey;
pub use self::domain_record::DomainRecord;

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