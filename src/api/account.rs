use super::{HasResponse, HasValue};
use crate::method::Get;
use crate::request::AccountRequest;
use crate::request::Request;
use crate::{ROOT_URL, STATIC_URL_ERROR};
use getset::{Getters, Setters};

const ACCOUNT_SEGMENT: &str = "account";

/// The user account.
///
/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#account)
#[derive(Deserialize, Serialize, Debug, Clone, Getters, Setters)]
pub struct Account {
    /// The total number of droplets the user may have.
    #[get = "pub"]
    droplet_limit: usize,
    /// The total number of floating IPs the user may have.
    #[get = "pub"]
    floating_ip_limit: usize,
    /// The email the user has registered for Digital Ocean with.
    #[get = "pub"]
    email: String,
    /// The universal identifier for this user.
    #[get = "pub"]
    uuid: String,
    /// If true, the user has verified their account via email. False otherwise.
    #[get = "pub"]
    email_verified: bool,
    /// This value is one of "active", "warning" or "locked".
    #[get = "pub"]
    status: String,
    /// A human-readable message giving more details about the status of the
    /// account.
    #[get = "pub"]
    status_message: String,
}

impl Account {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#get-user-information)
    pub fn get() -> AccountRequest<Get, Account> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(ACCOUNT_SEGMENT);

        Request::new(url)
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
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
