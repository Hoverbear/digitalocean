extern crate digitalocean;
#[macro_use]
extern crate log;
extern crate serde_json;
extern crate url;
extern crate url_serde;

mod utils;

use serde_json::Value;

use digitalocean::api::Account;
use digitalocean::request::Request;
use digitalocean::method::Get;

use utils::before;

#[test]
fn get_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/account";

    let req: Request<Get, Account> = Account::get();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}
