#[derive(Deserialize, Debug, Clone)]
pub struct Domain {
    pub name: String,
    pub ttl: Option<usize>,
    pub zone_file: Option<String>,
}