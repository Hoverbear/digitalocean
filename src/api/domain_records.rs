use std::fmt::Display;
use serde::Serialize;
use request::Request;
use action::{Get, List, Create};
use STATIC_URL_ERROR;
use url::Url;
use values::{DomainRecord, Domain};
use super::{ApiLinks, ApiMeta};
use super::{HasValue, HasPagination};

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

#[derive(Deserialize, Debug, Clone)]
pub struct DomainRecordsListResponse {
    domain_records: Vec<DomainRecord>,
    links: ApiLinks,
    meta: ApiMeta,
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

impl Request<Get, Domain> {
    pub fn records<'a>(&'a mut self) -> &'a mut Request<List, Vec<DomainRecord>> {
        self.url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push("records");

        // Yay type inferencing.
        self.action()
            .value()
    }
}

impl Request<List, Vec<DomainRecord>> {
    pub fn create<'a, S>(&'a mut self, kind: S, name: S, data: S) -> &'a mut Request<Create, DomainRecord>
    where S: AsRef<str> + Display + Serialize {
        self.url.path_segments_mut()
            .expect(STATIC_URL_ERROR);

        self.body(json!({
            "type": kind,
            "name": name,
            "data": data,
        }));

        self.action()
            .value()
    }

    pub fn get<'a>(&'a mut self, id: usize) -> &'a mut Request<Get, DomainRecord> {
        self.url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(&id.to_string());

        self.action()
            .value()
    }
}

impl Request<Create, DomainRecord> {
    pub fn priority<'a>(&'a mut self, val: Option<usize>) -> &'a mut Self {
        match &mut self.body {
            &mut Some(ref mut v) => v["priority"] = json!(val),
            &mut None => unreachable!("Should always have a body."),
        };
        self
    }
    pub fn port<'a>(&'a mut self, val: Option<usize>) -> &'a mut Self {
        match &mut self.body {
            &mut Some(ref mut v) => v["port"] = json!(val),
            &mut None => unreachable!("Should always have a body."),
        };
        self
    }
    pub fn ttl<'a>(&'a mut self, val: usize) -> &'a mut Self {
        match &mut self.body {
            &mut Some(ref mut v) => v["ttl"] = json!(val),
            &mut None => unreachable!("Should always have a body."),
        };
        self
    }
    pub fn weight<'a>(&'a mut self, val: Option<usize>) -> &'a mut Self {
        match &mut self.body {
            &mut Some(ref mut v) => v["weight"] = json!(val),
            &mut None => unreachable!("Should always have a body."),
        };
        self
    }
}