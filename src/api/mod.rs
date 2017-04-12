mod domain;

pub use self::domain::Domains;

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