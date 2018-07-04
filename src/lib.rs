/*!
A crate for interacting with the Digital Ocean API.

While browsing this documentation, please feel encouraged to reference the
[DigitalOcean docs](https://developers.digitalocean.com/documentation/v2/).

## A Basic Example

```rust,no_run
extern crate digitalocean;
use digitalocean::prelude::*;
use std::env;

fn main() {
    let api_key = env::var("API_KEY")
        .expect("API_KEY not set.");
    let client = DigitalOcean::new(api_key)
        .unwrap();

    Droplet::list()
        .execute(&client);
}
```

## Usage Fundamentals

All values (`Domain`, `SshKey`, etc) can be found in the `api` module.

Calling an action will return a `Request<_,_>` type. For example `Droplet::create()` will create a
`Request<Create, Droplet>`. These types may then have specific futher functions to futher build up
the request or transform it into some other request.

```rust,no_run
extern crate digitalocean;
use digitalocean::DigitalOcean;
use digitalocean::api::Domain;

fn main() {
    // Gets details of a specific domain.
    let req = Domain::get("foo.com");

    // Get the records for that domain instead (futher build the request)
    let req = req.records();
    // Get the records of a domain without having a prior request.
    let req = Domain::get("foo.com").records();

    // Create a new record for a domain
    let req = Domain::get("foo.com").records().create("CNAME", "test", "127.0.0.1");
}
```

In order to realize any action, `.execute()` must be called with a `DigitalOcean`
 client. It is also possible to call `do_client.execute(some_request)`.

In order to use the entire API, it is recommended to reference the various `Request` types.

## Design

The crate is founded on a few design considerations:

* Keep things simple and generic.
* Map closely to the DigitalOcean API.
* `Request`s are agnostic over `Client`s.
* It should be difficult to make an invalid API request.
* Use static dispatch as much as possible.
* Only the bare minimum amount of information should be carried around.
* Allow for easy construction of separate clients (`hyper`, etc.)
* No caching (yet). (DigitalOcean does not have [ETags](https://en.wikipedia.org/wiki/HTTP_ETag))

## Debugging

This crate uses the [`log`](https://doc.rust-lang.org/log/log/index.html) crate. You can see `digitalocean` logs by passing an environment variable such as:

```bash
RUST_LOG=digitalocean=debug cargo run
```

## Development Status

This crate is in a prototype state.

Not all endpoints have been fully end-to-end tested on the production DigitalOcean API. It's very
likely that some endpoints will have parsing errors due to unexpected values returned from the API.

**If something does not work please file a bug!**

Feedback, patches, and new features are encouraged.
Please just open an issue or PR!

*/

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate getset;
#[macro_use]
extern crate serde_json;
extern crate chrono;
extern crate failure;
extern crate serde;
extern crate url;
extern crate url_serde;
#[macro_use]
extern crate failure_derive;

pub mod api;
mod client;
pub mod error;
pub mod method;
pub mod prelude;
pub mod request;

use failure::Error;

use api::HasResponse;
use method::Method;
use request::{Executable, Request};
use url::Url;

const STATIC_URL_ERROR: &'static str = "Staticly constructed DigitalOcean URL is malformed.";
lazy_static! {
    static ref ROOT_URL: Url =
        Url::parse("https://api.digitalocean.com/v2").expect(STATIC_URL_ERROR);
}

/// A DigitalOcean Client that holds an API key.
#[derive(Clone)]
pub struct DigitalOcean {
    client: client::Client,
    token: String,
}

impl DigitalOcean {
    /// Create a DigitalOcean client with the given API key.
    pub fn new<T: Into<String>>(token: T) -> Result<Self, Error> {
        info!("Created.");
        Ok(DigitalOcean {
            client: client::Client::new()?,
            token: token.into(),
        })
    }

    pub fn execute<A, V>(&self, request: Request<A, V>) -> Result<V, Error>
    where
        A: Method,
        Request<A, V>: Executable<V>,
        V: HasResponse,
    {
        request.execute(self)
    }
}
