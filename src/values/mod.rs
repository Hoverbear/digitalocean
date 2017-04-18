mod domain;
pub use self::domain::Domain;

mod domain_record;
pub use self::domain_record::DomainRecord;

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