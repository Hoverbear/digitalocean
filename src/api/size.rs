use request::Request;
use action::List;
use {ROOT_URL, STATIC_URL_ERROR};
use url::Url;
use super::{ApiLinks, ApiMeta};
use super::{HasValue, HasPagination, HasResponse};

const IMAGES_SEGMENT: &'static str = "images";

/// The sizes objects represent different packages of hardware resources that 
/// can be used for Droplets. When a Droplet is created, a size must be 
/// selected so that the correct resources can be allocated.
///
/// Each size represents a plan that bundles together specific sets of 
/// resources. This includes the amount of RAM, the number of virtual CPUs,
/// disk space, and transfer. The size object also includes the pricing 
/// details and the regions that the size is available in.
///
/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#sizes)
#[derive(Deserialize, Debug, Clone)]
pub struct Size {
    /// A human-readable string that is used to uniquely identify each size.
    pub slug: String,
    /// This is a boolean value that represents whether new Droplets can be
    /// created with this size.
    pub available: bool,
    /// The amount of transfer bandwidth that is available for Droplets created
    /// in this size. This only counts traffic on the public interface. The 
    /// value is given in terabytes.
    pub transfer: usize,
    /// This attribute describes the monthly cost of this Droplet size if the
    /// Droplet is kept for an entire month. The value is measured in US 
    /// dollars.
    pub price_monthly: usize,
    /// This describes the price of the Droplet size as measured hourly. The
    /// value is measured in US dollars.
    pub price_hourly: usize,
    /// The amount of RAM allocated to Droplets created of this size. The value
    /// is represented in megabytes.
    pub memory: usize,
    /// The number of virtual CPUs allocated to Droplets of this size.
    pub vcpus: usize,
    /// The amount of disk space set aside for Droplets of this size. The value
    /// is represented in gigabytes.
    pub disk: usize,
    /// An array containing the region slugs where this size is available for 
    /// Droplet creates.
    pub regions: Vec<String>,
}

impl Size {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-images)
    pub fn list() -> Request<List, Vec<Size>> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(IMAGES_SEGMENT);

        Request::new(url)
    }
}

// There is no signular size return.

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Debug, Clone)]
pub struct SizeListResponse {
    sizes: Vec<Size>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasResponse for Vec<Size> {
    type Response = SizeListResponse;
}

impl HasPagination for SizeListResponse {
    fn next_page(&self) -> Option<Url> {
        self.links.next()
    }
}

impl HasValue for SizeListResponse {
    type Value = Vec<Size>;
    fn value(self) -> Vec<Size> {
        self.sizes
    }
}