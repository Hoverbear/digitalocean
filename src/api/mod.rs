mod domain;

use url_serde::SerdeUrl;
pub use self::domain::{Domains, Domain};

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
