mod domain;

pub use self::domain::{Domains, Domain};

// Defined in https://developers.digitalocean.com/documentation/v2/#links
pub const MAX_PER_PAGE: usize = 200;

#[derive(Deserialize, Debug)]
struct ApiLinks {
    pages: Option<ApiPages>,
}

#[derive(Deserialize, Debug)]
struct ApiPages {
    prev: Option<String>,
    first: Option<String>,
    next: Option<String>,
    last: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ApiMeta {
    total: usize,
}
