//! Domain record specific documentation.

use std::fmt::Display;
use serde::Serialize;
use request::Request;
use action::{Get, List, Create, Delete, Update};
use STATIC_URL_ERROR;
use url::Url;
use super::domain::Domain;
use super::{ApiLinks, ApiMeta};
use super::{HasValue, HasPagination, HasResponse};

const DOMAIN_RECORDS_SEGMENT: &'static str = "records";


/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
#[derive(Deserialize, Debug, Clone)]
pub struct DomainRecord {
    pub id: usize,
    #[serde(rename = "type")]
    pub kind: String, // 'type' is reserved in Rust.
    pub name: String,
    pub data: String,
    pub priority: Option<usize>,
    pub ttl: usize,
    pub port: Option<usize>,
    pub weight: Option<usize>,   
}

impl Request<Get, Domain> {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-domain-records)
    pub fn records(mut self) -> Request<List, Vec<DomainRecord>> {
        self.url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DOMAIN_RECORDS_SEGMENT);

        self.action()
            .value()
    }
}

impl Request<List, Vec<DomainRecord>> {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-domain-record)
    pub fn create<S>(mut self, kind: S, name: S, data: S) -> Request<Create, DomainRecord>
    where S: AsRef<str> + Display + Serialize {
        self.url.path_segments_mut()
            .expect(STATIC_URL_ERROR);

        self.body = json!({
            "type": kind,
            "name": name,
            "data": data,
        });

        self.action()
            .value()
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-domain-record)
    pub fn get(mut self, id: usize) -> Request<Get, DomainRecord> {
        self.url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(&id.to_string());

        self.action()
            .value()
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#update-a-domain-record)
    pub fn update(mut self, id: usize) -> Request<Update, DomainRecord> {
        self.url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(&id.to_string());

        self.action()
            .value()
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#delete-a-domain-record)
    pub fn delete(mut self, id: usize) -> Request<Delete, ()> {
        self.url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(&id.to_string());

        self.action()
            .value()
    }
}

impl Request<Create, DomainRecord> {
    /// The priority for SRV and MX records.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn priority(mut self, val: Option<usize>) -> Self {
        self.body["port"] = json!(val);
        self
    }
    /// The port for SRV records.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn port(mut self, val: Option<usize>) -> Self {
        self.body["port"] = json!(val);
        self
    }
    /// This value is the time to live for the record, in seconds. This defines
    /// the time frame that clients can cache queried information before a 
    /// refresh should be requested.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn ttl(mut self, val: usize) -> Self {
        self.body["ttl"] = json!(val);
        self
    }
    /// The weight for SRV records.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn weight(mut self, val: Option<usize>) -> Self {
        self.body["weight"] = json!(val);
        self
    }
}

impl Request<Update, DomainRecord> {
    /// The record type (A, MX, CNAME, etc).
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn kind<S>(mut self, val: S) -> Self
    where S: AsRef<str> + Display + Serialize {
        self.body["type"] = json!(val);
        self
    }
    /// The host name, alias, or service being defined by the record.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn name<S>(mut self, val: S) -> Self
    where S: AsRef<str> + Display + Serialize {
        self.body["name"] = json!(val);
        self
    }
    /// Variable data depending on record type. See the Domain Records section
    /// for more detail on each record type.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn data<S>(mut self, val: S) -> Self
    where S: AsRef<str> + Display + Serialize {
        self.body["data"] = json!(val);
        self
    }
    /// The priority for SRV and MX records.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn priority(mut self, val: Option<usize>) -> Self {
        self.body["priority"] = json!(val);
        self
    }
    /// The port for SRV records.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn port(mut self, val: Option<usize>) -> Self {
        self.body["port"] = json!(val);
        self
    }
    /// This value is the time to live for the record, in seconds. This defines
    /// the time frame that clients can cache queried information before a 
    /// refresh should be requested.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn ttl(mut self, val: usize) -> Self {
        self.body["ttl"] = json!(val);
        self
    }
    /// The weight for SRV records.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn weight(mut self, val: Option<usize>) -> Self {
        self.body["weight"] = json!(val);
        self
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Debug, Clone)]
pub struct DomainRecordsResponse {
    domain_record: DomainRecord,
}


impl HasValue for DomainRecordsResponse {
    type Value = DomainRecord;
    fn value(self) -> DomainRecord {
        self.domain_record
    }
}

impl HasResponse for DomainRecord {
    type Response = DomainRecordsResponse;
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Debug, Clone)]
pub struct DomainRecordsListResponse {
    domain_records: Vec<DomainRecord>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasResponse for Vec<DomainRecord> {
    type Response = DomainRecordsListResponse;
}

impl HasPagination for DomainRecordsListResponse {
    fn next_page(&self) -> Option<Url> {
        self.links.next()
    }
}

impl HasValue for DomainRecordsListResponse {
    type Value = Vec<DomainRecord>;
    fn value(self) -> Vec<DomainRecord> {
        self.domain_records
    }
}
