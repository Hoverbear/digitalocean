mod builder;
/// Later we can make a different client and implement it as a feature.
mod reqwest;

pub(crate) use self::builder::RequestBuilder;
pub use self::reqwest::Client;
