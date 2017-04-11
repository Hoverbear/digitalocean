mod domain;

pub use self::domain::Domains;
use std::marker::PhantomData;

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

struct RequestBuilder<T> {
    response_type: PhantomData<T>,
}