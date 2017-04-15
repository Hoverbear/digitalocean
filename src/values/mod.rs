mod domain;
pub use self::domain::Domain;
mod domain_record;
pub use self::domain_record::DomainRecord;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Image {
    pub id: usize,
    pub distribution: String,
    pub name: String,
    pub public: bool,
    pub regions: Vec<String>,
    pub min_disk_size: usize,
    pub size_gigabytes: f32,
    pub created_at: String,
    pub slug: Option<String>,
    #[serde(rename = "type")]
    pub kind: String, // 'type' is reserved in Rust.
}

#[derive(Deserialize, Debug, Clone)]
pub struct Region {
    pub name: String,
    pub slug: String,
    pub sizes: Vec<String>,
    pub features: Vec<String>,
    pub available: bool,
}

pub trait HasResponse {
    type Response: Deserialize + Clone;
}

impl HasResponse for () {
    type Response = ();
}