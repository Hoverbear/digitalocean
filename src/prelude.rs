//! Wildcard glob this module to have access to all commonly used items.

pub use DigitalOcean;
pub use api::{Account, Action, Certificate, Domain, DomainRecord, Droplet, FloatingIp, Image,
              LoadBalancer, Region, Size, Snapshot, SshKey, Tag, Volume};
pub use error::{Error, ErrorKind};
pub use request::Executable;
pub use request::Request;
