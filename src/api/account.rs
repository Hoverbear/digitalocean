use request::Request;
use method::Get;
use {ROOT_URL, STATIC_URL_ERROR};
use super::{HasValue, HasResponse};

const ACCOUNT_SEGMENT: &'static str = "account";

/// The user account.
///
/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#account)
#[derive(Deserialize, Debug, Clone)]
pub struct Account {
    /// The total number of droplets the user may have.
    pub droplet_limit: usize,
    /// The total number of floating IPs the user may have.
    pub floating_ip_limit: usize,
    /// The email the user has registered for Digital Ocean with.
    pub email: String,
    /// The universal identifier for this user.
    pub uuid: String,
    /// If true, the user has verified their account via email. False otherwise.
    pub email_verified: bool,
    /// This value is one of "active", "warning" or "locked".
    pub status: String,
    /// A human-readable message giving more details about the status of the 
    /// account.
    pub status_message: String,
}

impl Account {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#get-user-information)
    pub fn get() -> Request<Get, Account> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(ACCOUNT_SEGMENT);

        Request::new(url)
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Debug, Clone)]
pub struct AccountResponse {
    account: Account,
}

impl HasResponse for Account {
    type Response = AccountResponse;
}

impl HasValue for AccountResponse {
    type Value = Account;
    fn value(self) -> Account {
        self.account
    }
}