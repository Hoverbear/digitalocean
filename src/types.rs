use client::Client;
use error::{Result, Error};

#[derive(Deserialize, Debug)]
pub struct Image<'a> {
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

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    client: Option<&'a Client>,
}

#[derive(Deserialize, Debug)]
pub struct Region<'a> {
    pub name: String,
    pub slug: String,
    pub sizes: Vec<String>,
    pub features: Vec<String>,
    pub available: bool,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    client: Option<&'a Client>,
}

#[derive(Deserialize, Debug)]
pub struct Domain<'a> {
    pub name: String,
    pub ttl: Option<usize>,
    pub zone_file: Option<String>,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    client: Option<&'a Client>,
}

impl<'a> Domain<'a> {
    pub fn do_subresource_thing(&self, val: bool) -> Result<()> {
        let url = "blah";
        match self.client {
            Some(client) => {
                let request = client.get(url).send()?;
                Ok(())
            },
            None => Err(Error::MissingClient),
        }
    }

    pub fn set_client(&mut self, client: &'a Client) {
        self.client = Some(client);
    }
}