use super::HasResponse;
use api::domains::{DomainsResponse, DomainsListResponse};

impl HasResponse for Domain {
    type Response = DomainsResponse;
}

impl HasResponse for Vec<Domain> {
    type Response = DomainsListResponse;
}

#[derive(Deserialize, Debug, Clone)]
pub struct Domain {
    pub name: String,
    pub ttl: Option<usize>,
    pub zone_file: Option<String>,
}