extern crate digitalocean;
#[macro_use]
extern crate log;
extern crate serde_json;
extern crate url;
extern crate url_serde;

mod utils;

use serde_json::Value;

use digitalocean::api::Size;
use digitalocean::method::List;
use digitalocean::request::Request;

use crate::utils::before;

#[test]
fn list_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/sizes";

    let req: Request<List, Vec<Size>> = Size::list();
    info!("{:#?}", req);

    assert_eq!(req.url().as_str(), correct_url);
    assert_eq!(*req.body(), Value::Null);
}
