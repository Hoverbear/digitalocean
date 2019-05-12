extern crate digitalocean;

mod utils;

use digitalocean::prelude::{Bucket, ObjectACL, Requestable, Spaces};

#[test]
fn create_produces_correct_request() {
    crate::utils::before();

    let client = Spaces::new("foo", "bar");
    let b = Bucket::create("foobar", "nyc3").acl(ObjectACL::PublicRead);
    let req = b.build_request(&client).unwrap();

    assert_eq!(
        req.headers().get("x-amz-acl").unwrap().to_str().unwrap(),
        "public-read"
    );
    assert_eq!(*req.uri(), *"https://foobar.nyc3.digitaloceanspaces.com");
}
