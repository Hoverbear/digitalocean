
use types::Domain;
use reqwest::{Url, StatusCode};
use ROOT_URL;
use api::{ApiLinks, ApiMeta};
use client::Client;
use error::*;
use serde::Serialize;
use std::net::IpAddr;
use std::fmt::Display;

lazy_static! {
    static ref DOMAINS_URL: Url = ROOT_URL.join("domains/")
            .expect("This URL is static and should be well formed.");
}

#[derive(Deserialize, Debug)]
struct DomainsListResponse<'a> {
    domains: Vec<Domain<'a>>,
    links: ApiLinks,
    meta: ApiMeta,
}

#[derive(Deserialize, Debug)]
struct DomainsResponse<'a> {
    domain: Domain<'a>,
}

/// Accesses endpoints for domains.
///
/// [DigitalOcean documentation](https://developers.digitalocean.com/documentation/v2/#domains).
#[derive(Clone)]
pub struct Domains<'a> {
    client: &'a Client
}

impl<'a> Domains<'a> {
    #[doc(hidden)]
    pub fn new(client: &'a Client) -> Self {
        info!("Created");

        Domains { 
            client: client,
        }
    }

    /// Lists domains.
    ///
    /// [DigitalOcean documentation](https://developers.digitalocean.com/documentation/v2/#list-all-domains).
    pub fn list(&self) -> Result<Vec<Domain<'a>>> {
        info!("Listing.");

        let url = (*DOMAINS_URL).clone();
        // This request is not paginated.
        let mut request = self.client.get(url).send()?;
        
        match *request.status() {
            StatusCode::Ok => {
                let parsed: DomainsListResponse = request.json()?;
                let domains = parsed.domains.into_iter()
                    .map(|mut v| {
                        v.set_client(self.client);
                        v
                    }).collect();
                
                Ok(domains)
            },
            StatusCode::Unauthorized => Err(Error::Unauthorized)?,
            v => Err(Error::UnexpectedStatus(v))?,
        }
    }

    /// Creates a domain.
    ///
    /// [DigitalOcean documentation](https://developers.digitalocean.com/documentation/v2/#create-a-new-domain).
    pub fn create<N, I>(&self, name: N, ip_address: I) -> Result<Domain<'a>> 
    where N: AsRef<str> + Serialize + Display, I: Into<IpAddr> + Serialize + Display {
        info!("Creating {} ({}).", name, ip_address);

        let url = (*DOMAINS_URL).clone();

        let data = json!({
            "name": name.as_ref(),
            "ip_address": ip_address.into(),
        });

        let mut request = self.client.post(url).json(&data).send()?;

        match *request.status() {
            // Success
            StatusCode::Created => {
                let parsed: DomainsResponse = request.json()?;
                Ok(parsed.domain)
            },
            StatusCode::Unauthorized => Err(Error::Unauthorized)?,
            StatusCode::UnprocessableEntity => Err(Error::UnprocessableEntity)?,
            v => Err(Error::UnexpectedStatus(v))?,
        }
    }

    /// Gets a specific domain.
    ///
    /// [DigitalOcean documenation](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-domain).
    pub fn get<T>(&self, name: T) -> Result<Option<Domain<'a>>>
    where T: AsRef<str> + Display {
        info!("Getting {}.", name);

        let url = (*DOMAINS_URL).clone()
            .join(&format!("{}", name.as_ref()))?;

        let mut request = self.client.get(url).send()?;

        match *request.status() {
            // Success
            StatusCode::Ok => {
                let parsed: Option<DomainsResponse> = request.json()?;
                Ok(parsed.map(|v| v.domain))
            },
            // Failure
            StatusCode::Unauthorized => Err(Error::Unauthorized)?,
            v => Err(Error::UnexpectedStatus(v))?,
        }
    }

    /// Deletes a domain.
    ///
    /// [DigitalOcean documentation](https://developers.digitalocean.com/documentation/v2/#delete-a-domain).
    pub fn delete<T>(&self, name: T) -> Result<()>
    where T: AsRef<str> + Display {
        info!("Deleting {}.", name);

        let url = (*DOMAINS_URL).clone()
            .join(&format!("{}", name.as_ref()))?;

        let request = self.client.delete(url).send()?;

        match *request.status() {
            // Success
            StatusCode::NoContent => Ok(()),
            // Failure
            StatusCode::Unauthorized => Err(Error::Unauthorized)?,
            v => Err(Error::UnexpectedStatus(v))?,
        }
    }
}