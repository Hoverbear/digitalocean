//! Wildcard glob this module to have access to all commonly used items.

pub use DigitalOcean;
pub use api::{Account, Action, Certificate, DomainRecord, Domain, Droplet, FloatingIp, Image,
              LoadBalancer, Region, Size, Snapshot, SshKey, Tag, Volume};
pub use request::Request;
pub use request::Executable;
pub use error::{Error, ErrorKind};
