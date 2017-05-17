/*!
A crate for interacting with the Digital Ocean API.

While browsing this documentation please feel encouraged to reference the
[DigitalOcean docs](https://developers.digitalocean.com/documentation/v2/).

## A Basic Example

```rust,no_run
use digitalocean::DigitalOcean;
use digitalocean::request::Executable;
use digitalocean::api::Droplet;
use std::env;

let api_key = env::var("API_KEY")
    .expect("API_KEY not set.");
let client = DigitalOcean::new(api_key)
    .unwrap();

Droplet::list()
    .execute(&client);
```

## Usage Fundamentals

All values ([`Domain`](api/struct.Domain.html), [`SshKey`](api/struct.SshKey.html), etc) can 
be found in the [`api`](api/index.html) module.

Calling an action will return a [`Request<_,_>`](request/struct.Request.html) type. For example 
[`Droplet::create()`](api/struct.Droplet.html#method.create) will create a
[`Request<Create, Droplet>`](request/struct.Request.html#method.ssh_keys). This type may then
have specific futher functions to futher build up the request or transform it into some other
request.
In order to realize any action [`.execute()`](digitalocean/request/trait.Executable.html#tymethod.execute)
must be called with a [DigitalOcean](digitalocean/struct.DigitalOcean.html#method.new) client.

```rust,no_run
use digitalocean::DigitalOcean;
use digitalocean::api::Domain;

// Gets details of a specific domain.
let req = Domain::get("foo.com");

// Get the records for that domain instead (futher build the request)
let req = req.records();
// Get the records of a domain without having a prior request.
let req = Domain::get("foo.com").records();

// Create a new record for a domain
let req = Domain::get("foo.com").records().create("CNAME", "test", "127.0.0.1");
```

In order to use the entire API it is recommended to reference
[`Request<_,_>`](request/struct.Request.html) frequently.

## Design

The crate is founded on a few design considerations:

* Map closely to the DigitalOcean API.
* `Request`s are agnostic over `Client`s.
* It should be impossible to make an invalid API request.
* Use static dispatch as much as possible.
* Allow for easy construction of separate clients (`hyper`, etc.)


## Development Status

This library is in a prototype state.

Not all endpoints have been fully end-to-end tested on the production DigitalOcean API.
**If something does not work please file a bug!**

Feedback, patches, and new features are encouraged. 
Please just open an issue or PR!
*/

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate reqwest;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate serde;
extern crate url_serde;
extern crate url;
extern crate chrono;
#[macro_use] extern crate error_chain;

pub mod api;
mod error;
pub mod method;
pub mod request;
mod client;

use error::*;
pub use error::{Error, ErrorKind};

use request::{Request, Executable};
use method::Method;
use api::{HasResponse};
use url::Url;

const STATIC_URL_ERROR: &'static str = "Base DigitalOcean URL is malformed.";
lazy_static! {
    static ref ROOT_URL: Url = Url::parse("https://api.digitalocean.com/v2")
        .expect(STATIC_URL_ERROR);
}

/// A DigitalOcean Client that holds an API key.
#[derive(Clone)]
pub struct DigitalOcean {
    client: client::Client,
    token: String,
}

impl DigitalOcean {
    /// Create a DigitalOcean client with the given API key.
    pub fn new<T: Into<String>>(token: T) -> Result<Self> {
        info!("Created.");
        Ok(DigitalOcean {
            client: client::Client::new()?,
            token: token.into(),
        })
    }

    pub fn execute<A,V>(&self, request: Request<A,V>) -> Result<V>
    where A: Method, 
          Request<A,V>: Executable<V>,
          V: HasResponse {
        request.execute(self)
    }
}
