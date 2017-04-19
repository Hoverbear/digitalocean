/// Images in DigitalOcean may refer to one of a few different kinds of objects.
///
/// An image may refer to a snapshot that has been taken of a Droplet instance.
/// It may also mean an image representing an automatic backup of a Droplet. 
/// The third category that it can represent is a public Linux distribution or
/// application image that is used as a base to create Droplets.
///
/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domains)
#[derive(Deserialize, Debug, Clone)]
pub struct Image {
    /// A unique number that can be used to identify and reference a specific
    /// image.
    pub id: usize,
    /// The display name that has been given to an image. This is what is shown
    /// in the control panel and is generally a descriptive title for the image
    /// in question.
    pub name: String,
    /// The kind of image, describing the duration of how long the image is 
    /// stored. This is either "snapshot" or "backup".
    ///
    /// *Note:* Since `type` is a keyword in Rust `kind` is used instead.
    #[serde(rename = "type")]
    pub kind: String, // 'type' is reserved in Rust.
    /// This attribute describes the base distribution used for this image.
    pub distribution: String,
    /// A uniquely identifying string that is associated with each of the
    /// DigitalOcean-provided public images. These can be used to reference
    /// a public image as an alternative to the numeric id.
    pub slug: Option<String>,
    /// This is a boolean value that indicates whether the image in question
    /// is public or not. An image that is public is available to all accounts.
    /// A non-public image is only accessible from your account.
    pub public: bool,
    /// This attribute is an array of the regions that the image is available
    /// in. The regions are represented by their identifying slug values.
    pub regions: Vec<String>,
    /// The minimum 'disk' required for a size to use this image.
    pub min_disk_size: usize,
    /// The size of the image in gigabytes.
    pub size_gigabytes: f32,
    /// A time value given in ISO8601 combined date and time format that 
    /// represents when the Image was created.
    pub created_at: String,
}