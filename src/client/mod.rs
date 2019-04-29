#[cfg(feature = "spaces")]
mod builder;
/// Later we can make a different client and implement it as a feature.
mod reqwest;
#[cfg(feature = "spaces")]
mod spaces;

#[cfg(feature = "spaces")]
pub(crate) use self::builder::RequestBuilder;
pub use self::reqwest::Client;
#[cfg(feature = "spaces")]
pub use self::spaces::Spaces;
