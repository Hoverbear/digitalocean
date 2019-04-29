use super::object::ObjectACL;
use super::SpacesRequest;
use crate::client::ACL_HEADER;
use crate::method::Update;
use crate::Spaces;
use failure::Error;
use http::method::Method;

/// Represents a DigitalOcean bucket.
///
/// https://developers.digitalocean.com/documentation/spaces/#bucket-ops
#[derive(Clone)]
pub struct Bucket {
    /// Name of this bucket.
    name: String,
    /// Region of this bucket.
    region: String,
    /// ACL rule for this bucket.
    acl: Option<ObjectACL>,
}

impl Bucket {
    /// Initialize an object to create a space / bucket.
    pub fn create<N, R>(name: N, region: R) -> SpacesRequest<Self, Update, ()>
    where
        N: Into<String>,
        R: Into<String>,
    {
        SpacesRequest::from_model(Bucket {
            name: name.into(),
            region: region.into(),
            acl: None,
        })
    }
}

impl SpacesRequest<Bucket, Update, ()> {
    /// Set the access control for this bucket.
    pub fn acl(mut self, acl: ObjectACL) -> Self {
        self.model.acl = Some(acl);
        self
    }

    pub async fn execute(self, client: &Spaces) -> Result<(), Error> {
        let mut builder = client.builder(Method::PUT, &self.model.region, &self.model.name);
        if let Some(value) = self.model.acl {
            builder.headers.insert(ACL_HEADER.clone(), value.header());
        }

        await!(client.fetch_response(builder))?;
        Ok(())
    }
}
