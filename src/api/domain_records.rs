use request::Request;
use action::{Get, Post, Delete};
use {ROOT_URL, STATIC_URL_ERROR};
use url::Url;
use values::{DomainRecord, Domain};
use super::{ApiLinks, ApiMeta, MAX_PER_PAGE};
use super::{HasValue, HasPagination};


#[derive(Deserialize, Debug, Clone)]
pub struct DomainRecordsResponse {
    domain_records: Vec<DomainRecord>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasPagination for DomainRecordsResponse {
    fn next_page(&self) -> Option<Url> {
        match self.links.pages {
            Some(ref pages) => match pages.next {
                Some(ref v) => Some(v.clone().into_inner()),
                None => None,
            },
            None => None,
        }
    }
}

impl HasValue for DomainRecordsResponse {
    type Value = Vec<DomainRecord>;
    fn value(self) -> Vec<DomainRecord> {
        self.domain_records
    }
}

impl Request<Get, Domain> {
    pub fn records<'a>(&'a mut self) -> &'a mut Request<Get, Vec<DomainRecord>> {
        self.url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push("records");
        // Safe because we're only changing PhantomData.
        unsafe {
            &mut *(self as *mut _ as *mut Request<Get, Vec<DomainRecord>>)
        }
    }
}