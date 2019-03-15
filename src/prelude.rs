//! Wildcard glob this module to have access to all commonly used items.

pub use crate::api::{
    Account, Action, Certificate, CustomImage, Domain, DomainRecord, Droplet, FloatingIp, Image,
    LoadBalancer, Region, Size, Snapshot, SshKey, Tag, Volume,
};
pub use crate::request::Executable;
pub use crate::request::Request;
pub use crate::DigitalOcean;
