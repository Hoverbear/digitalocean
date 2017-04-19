//! Values Domain Record documentation.

use super::HasResponse;
use api::domain_records::{DomainRecordsResponse, DomainRecordsListResponse};

/// https://developers.digitalocean.com/documentation/v2/#domain-records
#[derive(Deserialize, Debug, Clone)]
pub struct DomainRecord {
    pub id: usize,
    #[serde(rename = "type")]
    pub kind: String, // 'type' is reserved in Rust.
    pub name: String,
    pub data: String,
    pub priority: Option<usize>,
    pub ttl: usize,
    pub port: Option<usize>,
    pub weight: Option<usize>,   
}

impl HasResponse for DomainRecord {
    type Response = DomainRecordsResponse;
}

impl HasResponse for Vec<DomainRecord> {
    type Response = DomainRecordsListResponse;
}