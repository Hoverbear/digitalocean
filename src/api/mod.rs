pub mod domains;
pub mod domain_records;

use url::Url;
use serde::Deserialize;
use url_serde::SerdeUrl;
pub use self::domains::Domains;

// Defined in https://developers.digitalocean.com/documentation/v2/#links
pub const MAX_PER_PAGE: usize = 200;

#[derive(Deserialize, Debug, Clone)]
struct ApiLinks {
    pages: Option<ApiPages>,
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