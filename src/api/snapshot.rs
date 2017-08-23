use super::{ApiLinks, ApiMeta};
use super::{HasPagination, HasResponse, HasValue};
use {ROOT_URL, STATIC_URL_ERROR};
use chrono::{DateTime, Utc};
use method::{Delete, Get, List};
use request::Request;
use request::SnapshotRequest;
use url::Url;

const SNAPSHOT_SEGMENT: &'static str = "snapshots";

/// Snapshots are saved instances of a Droplet or a volume, which is reflected
/// in the `resource_type` attribute. In order to avoid problems with
/// compressing filesystems, each defines a `min_disk_size` attribute which is
/// the minimum size of the Droplet or volume disk when creating a new resource
/// from the saved snapshot.
///
/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#snapshots)
#[derive(Deserialize, Serialize, Debug, Clone, Getters, Setters)]
pub struct Snapshot {
    /// The unique identifier for the snapshot.
    #[get = "pub"]
    id: String,
    /// A human-readable name for the snapshot.
    #[get = "pub"]
    name: String,
    /// A time value given in ISO8601 combined date and time format that
    /// represents when the snapshot was created.
    #[get = "pub"]
    created_at: DateTime<Utc>,
    /// An array of the regions that the image is available in. The regions
    /// are represented by their identifying slug values.
    #[get = "pub"]
    regions: Vec<String>,
    /// A unique identifier for the resource that the action is associated
    /// with.
    #[get = "pub"]
    resource_id: String,
    /// The type of resource that the action is associated with.
    #[get = "pub"]
    resource_type: String,
    /// The minimum size in GB required for a volume or Droplet to use this snapshot.
    #[get = "pub"]
    min_disk_size: usize,
    /// The billable size of the snapshot in gigabytes.
    #[get = "pub"]
    size_gigabytes: usize,
}

impl Snapshot {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-snapshots)
    pub fn list() -> SnapshotRequest<List, Vec<Snapshot>> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut().expect(STATIC_URL_ERROR).push(
            SNAPSHOT_SEGMENT,
        );

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-droplet-snapshots)
    pub fn droplets() -> SnapshotRequest<List, Vec<Snapshot>> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut().expect(STATIC_URL_ERROR).push(
            SNAPSHOT_SEGMENT,
        );

        url.query_pairs_mut().append_pair(
            "resource_type",
            "droplet",
        );

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-volume-snapshots)
    pub fn volumes() -> SnapshotRequest<List, Vec<Snapshot>> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut().expect(STATIC_URL_ERROR).push(
            SNAPSHOT_SEGMENT,
        );

        url.query_pairs_mut().append_pair("resource_type", "volume");

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-snapshot-by-id)
    pub fn get(id: usize) -> SnapshotRequest<Get, Snapshot> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(SNAPSHOT_SEGMENT)
            .push(&id.to_string());

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#delete-a-snapshot)
    pub fn delete(id: usize) -> SnapshotRequest<Delete, ()> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(SNAPSHOT_SEGMENT)
            .push(&id.to_string());

        Request::new(url)
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SnapshotListResponse {
    snapshots: Vec<Snapshot>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasResponse for Vec<Snapshot> {
    type Response = SnapshotListResponse;
}


impl HasPagination for SnapshotListResponse {
    fn next_page(&self) -> Option<Url> {
        self.links.next()
    }
}

impl HasValue for SnapshotListResponse {
    type Value = Vec<Snapshot>;
    fn value(self) -> Vec<Snapshot> {
        self.snapshots
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SnapshotResponse {
    snapshot: Snapshot,
}

impl HasResponse for Snapshot {
    type Response = SnapshotResponse;
}

impl HasValue for SnapshotResponse {
    type Value = Snapshot;
    fn value(self) -> Snapshot {
        self.snapshot
    }
}
