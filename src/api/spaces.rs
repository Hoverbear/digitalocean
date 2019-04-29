/// Response returned by Spaces API whenever an error occurs.
#[derive(Debug, Deserialize)]
pub struct SpacesError {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "RequestId")]
    pub request_id: Option<String>,
    #[serde(rename = "HostId")]
    pub host_id: Option<String>,
}
