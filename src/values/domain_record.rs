use super::HasResponse;
use api::domain_records::DomainRecordsResponse;

#[derive(Deserialize, Debug, Clone)]
pub struct DomainRecord {
    id: usize,
    #[serde(rename = "type")]
    kind: String, // 'type' is reserved in Rust.
    name: String,
    data: String,
    priority: Option<usize>,
    port: Option<usize>,
    weight: Option<usize>,
    
}

impl HasResponse for Vec<DomainRecord> {
    type Response = DomainRecordsResponse;
}