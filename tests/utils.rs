extern crate dotenv;
extern crate env_logger;

pub fn before() {
    // Setup for tests
    dotenv::dotenv().ok();
    env_logger::try_init().ok();
}
