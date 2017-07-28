
use self::droplet_fields::{Kernel, Networks, NextBackupWindow};
use super::{ApiLinks, ApiMeta};
use super::{HasPagination, HasResponse, HasValue};
use super::{Image, Region, Size};
use super::snapshot::Snapshot;
use {ROOT_URL, STATIC_URL_ERROR};
use chrono::{DateTime, Utc};
use method::{Create, Delete, Get, List};
use request::{DropletRequest, SnapshotRequest};
use request::Request;
use serde::Serialize;
use std::fmt::Display;
use url::Url;

const DROPLETS_SEGMENT: &'static str = "droplets";
const REPORTS_SEGMENT: &'static str = "reports";
const DROPLET_NEIGHBORS_SEGMENT: &'static str = "droplet_neighbors";
const NEIGHBORS_SEGMENT: &'static str = "neighbors";
const SNAPSHOTS_SEGMENT: &'static str = "snapshots";
const BACKUPS_SEGMENT: &'static str = "backups";

/// A Droplet is a DigitalOcean virtual machine. By sending requests to the
/// Droplet endpoint, you can list, create, or delete Droplets.
///
/// Some of the attributes will have an object value. The region and image
/// objects will all contain the standard attributes of their associated types.
/// Find more information about each of these objects in their respective
/// sections.
///
/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domains)
#[derive(Deserialize, Serialize, Debug, Clone, Getters, Setters)]
pub struct Droplet {
    /// A unique identifier for each Droplet instance. This is automatically
    /// generated upon Droplet creation.
    #[get = "pub"]
    id: usize,
    /// The human-readable name set for the Droplet instance.
    #[get = "pub"]
    name: String,
    /// Memory of the Droplet in megabytes.
    #[get = "pub"]
    memory: usize,
    /// The number of virtual CPUs.
    #[get = "pub"]
    vcpus: usize,
    /// The size of the Droplet's disk in gigabytes.
    #[get = "pub"]
    disk: usize,
    /// A boolean value indicating whether the Droplet has been locked,
    /// preventing actions by users.
    #[get = "pub"]
    locked: bool,
    /// A time value given in ISO8601 combined date and time format that
    /// represents when the Droplet was created.
    #[get = "pub"]
    created_at: DateTime<Utc>,
    /// A status string indicating the state of the Droplet instance. This may
    /// be "new", "active", "off", or "archive".
    #[get = "pub"]
    status: String,
    /// An array of backup IDs of any backups that have been taken of the
    /// Droplet instance. Droplet backups are enabled at the time of the
    /// instance creation.
    #[get = "pub"]
    backup_ids: Vec<usize>,
    /// An array of snapshot IDs of any snapshots created from the Droplet
    /// instance.
    #[get = "pub"]
    snapshot_ids: Vec<usize>,
    /// An array of features enabled on this Droplet.
    #[get = "pub"]
    features: Vec<String>,
    /// The region that the Droplet instance is deployed in. When setting a
    /// region, the value should be the slug identifier for the region. When
    /// you query a Droplet, the entire region object will be returned.
    #[get = "pub"]
    region: Region,
    /// The base image used to create the Droplet instance. When setting an
    /// image, the value is set to the image id or slug. When querying the
    /// Droplet, the entire image object will be returned.
    #[get = "pub"]
    image: Image,
    /// The current size object describing the Droplet. When setting a size,
    /// the value is set to the size slug. When querying the Droplet, the
    /// entire size object will be returned. Note that the disk volume of a
    /// Droplet may not match the size's disk due to Droplet resize actions.
    /// The disk attribute on the Droplet should always be referenced.
    #[get = "pub"]
    size: Size,
    /// The unique slug identifier for the size of this Droplet.
    #[get = "pub"]
    size_slug: String,
    /// The details of the network that are configured for the Droplet
    /// instance. This is an object that contains keys for IPv4 and IPv6.
    /// The value of each of these is an array that contains objects describing
    /// an individual IP resource allocated to the Droplet. These will define
    /// attributes like the IP address, netmask, and gateway of the specific
    /// network depending on the type of network it is.
    #[get = "pub"]
    networks: Networks,
    /// The current kernel. This will initially be set to the kernel of the
    /// base image when the Droplet is created.
    #[get = "pub"]
    kernel: Option<Kernel>,
    /// The details of the Droplet's backups feature, if backups are configured
    /// for the Droplet. This object contains keys for the start and end times
    /// of the window during which the backup will start.
    #[get = "pub"]
    next_backup_window: Option<NextBackupWindow>,
    /// An array of Tags the Droplet has been tagged with.
    #[get = "pub"]
    tags: Vec<String>,
    /// A flat array including the unique identifier for each Block Storage
    /// volume attached to the Droplet.
    #[get = "pub"]
    volume_ids: Vec<String>,
}

/// Fields which exists inside Droplets.
pub mod droplet_fields {
    use chrono::{DateTime, Utc};
    use std::net::IpAddr;
    /// This exists in the `networks` field of a droplet.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Networks {
        pub v4: Vec<Network>,
        pub v6: Vec<Network>,
    }

    /// These exist in the `networks` field of a droplet.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Network {
        pub gateway: IpAddr,
        pub ip_address: IpAddr,
        pub netmask: IpAddr,
        /// *Note:* Since `type` is a keyword in Rust `kind` is used instead.
        #[serde(rename = "type")]
        pub kind: String,
    }

    /// This exists in the `next_backup_window` field of a droplet.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct NextBackupWindow {
        pub end: DateTime<Utc>,
        pub start: DateTime<Utc>,
    }

    /// This exists in the `kernel` field of a droplet.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Kernel {
        pub id: usize,
        pub name: String,
        pub version: String,
    }
}

impl Droplet {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-droplet)
    pub fn create<S, D>(name: S, region: S, size: S, image: D) -> DropletRequest<Create, Droplet>
    where
        S: AsRef<str> + Serialize + Display,
        D: Serialize + Display,
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut().expect(STATIC_URL_ERROR).push(
            DROPLETS_SEGMENT,
        );

        let mut req = Request::new(url);
        req.set_body(json!({
            "name": name,
            "region": region,
            "size": size,
            "image": format!("{}", image),
        }));
        req
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-multiple-droplets)
    pub fn create_multiple<S, D>(
        names: Vec<S>,
        region: S,
        size: S,
        image: D,
    ) -> DropletRequest<Create, Vec<Droplet>>
    where
        S: AsRef<str> + Serialize + Display,
        D: Serialize + Display,
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut().expect(STATIC_URL_ERROR).push(
            DROPLETS_SEGMENT,
        );

        let mut req = Request::new(url);
        req.set_body(json!({
            "names": names,
            "region": region,
            "size": size,
            "image": format!("{}", image),
        }));
        req
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-droplet-by-id)
    pub fn get(id: usize) -> DropletRequest<Get, Droplet> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLETS_SEGMENT)
            .push(&id.to_string());

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-droplets)
    pub fn list() -> DropletRequest<List, Vec<Droplet>> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut().expect(STATIC_URL_ERROR).push(
            DROPLETS_SEGMENT,
        );

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#listing-droplets-by-tag)
    pub fn list_by_tag<S>(name: S) -> DropletRequest<List, Vec<Droplet>>
    where
        S: AsRef<str> + Serialize,
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut().expect(STATIC_URL_ERROR).push(
            DROPLETS_SEGMENT,
        );

        url.query_pairs_mut().append_pair("tag_name", name.as_ref());

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#delete-a-droplet)
    pub fn delete(id: usize) -> DropletRequest<Delete, ()> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLETS_SEGMENT)
            .push(&id.to_string());

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#deleting-droplets-by-tag)
    pub fn delete_by_tag<S>(name: S) -> DropletRequest<Delete, ()>
    where
        S: AsRef<str> + Serialize,
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut().expect(STATIC_URL_ERROR).push(
            DROPLETS_SEGMENT,
        );

        url.query_pairs_mut().append_pair("tag_name", name.as_ref());

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-droplet-neighbors)
    pub fn neighbors() -> DropletRequest<Get, Vec<Vec<Droplet>>> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(REPORTS_SEGMENT)
            .push(DROPLET_NEIGHBORS_SEGMENT);

        Request::new(url)
    }
}

impl DropletRequest<Create, Droplet> {
    /// An array containing the IDs or fingerprints of the SSH keys that you
    /// wish to embed in the Droplet's root account upon creation.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-droplet)
    pub fn ssh_keys<D>(mut self, val: Vec<D>) -> Self
    where
        D: Display + Serialize,
    {
        self.body_mut()["ssh_keys"] = json!(val);
        self
    }
    /// A boolean indicating whether automated backups should be enabled for
    /// the Droplet. Automated backups can only be enabled when the Droplet is
    /// created.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-droplet)
    pub fn backups(mut self, val: bool) -> Self {
        self.body_mut()["backups"] = json!(val);
        self
    }
    /// A boolean indicating whether IPv6 is enabled on the Droplet.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-droplet)
    pub fn ipv6(mut self, val: bool) -> Self {
        self.body_mut()["ipv6"] = json!(val);
        self
    }
    /// A boolean indicating whether private networking is enabled for the
    /// Droplet. Private networking is currently only available in certain
    /// regions.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-droplet)
    pub fn private_networking(mut self, val: bool) -> Self {
        self.body_mut()["private_networking"] = json!(val);
        self
    }
    /// A string containing 'user data' which may be used to configure the
    /// Droplet on first boot, often a 'cloud-config' file or Bash script.
    /// It must be plain text and may not exceed 64 KiB in size.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-droplet)
    pub fn user_data(mut self, val: bool) -> Self {
        self.body_mut()["user_data"] = json!(val);
        self
    }
    /// A boolean indicating whether to install the DigitalOcean agent
    /// for monitoring.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-droplet)
    pub fn monitoring(mut self, val: bool) -> Self {
        self.body_mut()["monitoring"] = json!(val);
        self
    }
    /// A flat array including the unique string identifier for each Block
    /// Storage volume to be attached to the Droplet. At the moment a volume
    /// can only be attached to a single Droplet.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-droplet)
    pub fn volumes(mut self, val: Vec<String>) -> Self {
        self.body_mut()["volumes"] = json!(val);
        self
    }
    /// A flat array of tag names as strings to apply to the Droplet after it
    /// is created. Tag names can either be existing or new tags.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-droplet)
    pub fn tags(mut self, val: Vec<String>) -> Self {
        self.body_mut()["tags"] = json!(val);
        self
    }
}


impl DropletRequest<Create, Vec<Droplet>> {
    /// An array containing the IDs or fingerprints of the SSH keys that you
    /// wish to embed in the Droplet's root account upon creation.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-droplet)
    pub fn ssh_keys<D>(mut self, val: Vec<D>) -> Self
    where
        D: Display + Serialize,
    {
        self.body_mut()["ssh_keys"] = json!(val);
        self
    }
    /// A boolean indicating whether automated backups should be enabled for
    /// the Droplet. Automated backups can only be enabled when the Droplet is
    /// created.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-droplet)
    pub fn backups(mut self, val: bool) -> Self {
        self.body_mut()["backups"] = json!(val);
        self
    }
    /// A boolean indicating whether IPv6 is enabled on the Droplet.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-droplet)
    pub fn ipv6(mut self, val: bool) -> Self {
        self.body_mut()["ipv6"] = json!(val);
        self
    }
    /// A boolean indicating whether private networking is enabled for the
    /// Droplet. Private networking is currently only available in certain
    /// regions.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-droplet)
    pub fn private_networking(mut self, val: bool) -> Self {
        self.body_mut()["private_networking"] = json!(val);
        self
    }
    /// A string containing 'user data' which may be used to configure the
    /// Droplet on first boot, often a 'cloud-config' file or Bash script.
    /// It must be plain text and may not exceed 64 KiB in size.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-droplet)
    pub fn user_data(mut self, val: bool) -> Self {
        self.body_mut()["user_data"] = json!(val);
        self
    }
    /// A boolean indicating whether to install the DigitalOcean agent
    /// for monitoring.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-droplet)
    pub fn monitoring(mut self, val: bool) -> Self {
        self.body_mut()["monitoring"] = json!(val);
        self
    }
    /// A flat array including the unique string identifier for each Block
    /// Storage volume to be attached to the Droplet. At the moment a volume
    /// can only be attached to a single Droplet.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-droplet)
    pub fn volumes(mut self, val: Vec<String>) -> Self {
        self.body_mut()["volumes"] = json!(val);
        self
    }
    /// A flat array of tag names as strings to apply to the Droplet after it
    /// is created. Tag names can either be existing or new tags.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-droplet)
    pub fn tags(mut self, val: Vec<String>) -> Self {
        self.body_mut()["tags"] = json!(val);
        self
    }
}

impl DropletRequest<Get, Droplet> {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-snapshots-for-a-droplet)
    pub fn snapshots(mut self) -> SnapshotRequest<List, Vec<Snapshot>> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(SNAPSHOTS_SEGMENT);

        self.transmute()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-backups-for-a-droplet)
    pub fn backups(mut self) -> SnapshotRequest<List, Vec<Snapshot>> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(BACKUPS_SEGMENT);

        self.transmute()
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-neighbors-for-a-droplet)
    pub fn neighbors(mut self) -> DropletRequest<List, Vec<Droplet>> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(NEIGHBORS_SEGMENT);

        self.transmute()
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DropletResponse {
    droplet: Droplet,
}

impl HasResponse for Droplet {
    type Response = DropletResponse;
}

impl HasValue for DropletResponse {
    type Value = Droplet;
    fn value(self) -> Droplet {
        self.droplet
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DropletListResponse {
    droplets: Vec<Droplet>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasResponse for Vec<Droplet> {
    type Response = DropletListResponse;
}

impl HasPagination for DropletListResponse {
    fn next_page(&self) -> Option<Url> {
        self.links.next()
    }
}

impl HasValue for DropletListResponse {
    type Value = Vec<Droplet>;
    fn value(self) -> Vec<Droplet> {
        self.droplets
    }
}

/// Response type returned from Digital Ocean
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DropletNeighborsResponse {
    neighbors: Vec<Vec<Droplet>>,
}

impl HasResponse for Vec<Vec<Droplet>> {
    type Response = DropletNeighborsResponse;
}

impl HasValue for DropletNeighborsResponse {
    type Value = Vec<Vec<Droplet>>;
    fn value(self) -> Vec<Vec<Droplet>> {
        self.neighbors
    }
}
