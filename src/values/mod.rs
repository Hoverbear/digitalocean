// Values used and returned by the API.

mod domain;
pub use self::domain::Domain;

mod domain_record;
pub use self::domain_record::DomainRecord;

mod ssh_key;
pub use self::ssh_key::SshKey;

mod image;
pub use self::image::Image;

mod region;
pub use self::region::Region;

use serde::Deserialize;

pub trait HasResponse {
    type Response: Deserialize + Clone;
}

impl HasResponse for () {
    type Response = ();
}