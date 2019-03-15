extern crate digitalocean;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
extern crate url;
extern crate url_serde;

mod utils;

use digitalocean::api::CustomImage;
use digitalocean::method::Create;
use digitalocean::request::Request;

use crate::utils::before;

#[test]
fn create_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/images";

    let req: Request<Create, CustomImage> = CustomImage::create(
        "test",
        "http://foo.bar/baz.raw",
        "ams3",
        "archlinux",
        "test image",
        vec!["arch", "small"],
    );
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(
        *req.body(),
        json!({
            "name": "test",
            "url": "http://foo.bar/baz.raw",
            "region": "ams3",
            "distribution": "archlinux",
            "description": "test image",
            "tags": vec!["arch", "small"]
        })
    );
}
