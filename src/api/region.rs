#[derive(Deserialize, Debug, Clone)]
pub struct Region {
    pub name: String,
    pub slug: String,
    pub sizes: Vec<String>,
    pub features: Vec<String>,
    pub available: bool,
}
