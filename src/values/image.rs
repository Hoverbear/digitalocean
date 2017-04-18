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