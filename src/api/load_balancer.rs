use serde::Serialize;
use std::fmt::Display;
use std::net::IpAddr;
use request::Request;
use method::{List, Get, Create, Update, Delete};
use {ROOT_URL, STATIC_URL_ERROR};
use url::Url;
use chrono::{DateTime, UTC};
use super::Region;
use super::{ApiLinks, ApiMeta};
use super::{HasValue, HasPagination, HasResponse};
use self::load_balancer_fields::{ForwardingRule, HealthCheck, StickySessions};

const LOAD_BALANCERS_SEGMENT: &'static str = "load_balancers";
const DROPLETS_SEGMENT: &'static str = "droplets";
const FORWARDING_RULES_SEGMENT: &'static str = "forwarding_rules";

/// Load Balancers provide a way to distribute traffic across multiple
/// Droplets.
///
/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#load-balancers)
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LoadBalancer {
    /// A unique ID that can be used to identify and reference a Load Balancer.
    pub id: String,
    /// A human-readable name for a Load Balancer instance.
    pub name: String,
    /// An attribute containing the public-facing IP address of the Load
    /// Balancer.
    pub ip: IpAddr,
    /// The load balancing algorithm used to determine which backend Droplet
    /// will be selected by a client. It must be either "round_robin" or
    /// "least_connections".
    pub algorithm: String,
    /// A status string indicating the current state of the Load Balancer.
    /// This can be "new", "active", or "errored".
    pub status: String,
    /// A time value given in ISO8601 combined date and time format that
    /// represents when the Load Balancer was created.
    pub created_at: DateTime<UTC>,
    /// An object specifying the forwarding rules for a Load Balancer.
    pub forwarding_rules: Vec<ForwardingRule>,
    /// An object specifying health check settings for the Load Balancer.
    pub health_check: HealthCheck,
    /// An object specifying sticky sessions settings for the Load Balancer.
    pub sticky_sessions: StickySessions,
    /// The region where the Load Balancer instance is located.
    pub region: Region,
    /// The name of a Droplet tag corresponding to Droplets assigned to the
    /// Load Balancer.
    pub tag: String,
    /// An array containing the IDs of the Droplets assigned to the Load
    /// Balancer.
    pub droplet_ids: Vec<usize>,
    /// A boolean value indicating whether HTTP requests to the Load Balancer
    /// on port 80 will be redirected to HTTPS on port 443.
    pub redirect_http_to_https: bool,
}

/// Fields which exists inside Droplets.
pub mod load_balancer_fields {
    /// This exists in the `forwarding_rules` field of a droplet.
    ///
    /// Forwarding rules determine how traffic will be routed from the Load
    /// Balancer to the Droplets assigned to it. They can be used to configure
    /// the type of traffic (HTTP, HTTPS, or TCP) and to map ports on the Load
    /// Balancer to ports on the Droplets. For SSL encrypted traffic, you may
    /// also configure whether to use SSL termination at the Load Balancer (by
    /// specifying an SSL certificate) or to pass the encrypted traffic
    /// through to the Droplet. Currently, each Load Balancer may have up to 15
    /// forwarding rules.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct ForwardingRule {
        /// The protocol used for traffic to the Load Balancer. The possible
        /// values are: "http", "https", or "tcp".
        pub entry_protocol: String,
        /// The port on which the Load Balancer instance will listen.
        pub entry_port: usize,
        /// The protocol used for traffic from the Load Balancer to the backend
        /// Droplets. The possible values are: "http", "https", or "tcp".
        pub target_protocol: String,
        /// An integer representing the port on the backend Droplets to which
        /// the Load Balancer will send traffic.
        pub target_port: usize,
        /// The ID of the TLS certificate used for SSL termination if enabled.
        pub certificate_id: Option<String>,
        /// A boolean value indicating whether SSL encrypted traffic will be
        /// passed through to the backend Droplets.
        pub tls_passthrough: bool,
    }
    impl ForwardingRule {
        pub fn new<S>(entry_protocol: S,
                      entry_port: usize,
                      target_protocol: S,
                      target_port: usize)
                      -> Self
            where S: AsRef<str>
        {
            ForwardingRule {
                entry_protocol: entry_protocol.as_ref().to_string(),
                entry_port: entry_port,
                target_protocol: target_protocol.as_ref().to_string(),
                target_port: target_port,
                certificate_id: None,
                tls_passthrough: false,
            }
        }
        pub fn certificate_id<S>(mut self, certificate_id: Option<S>) -> Self
            where S: AsRef<str>
        {
            self.certificate_id = certificate_id.map(|v| v.as_ref().to_string());
            self
        }
        pub fn tls_passthrough(mut self, tls_passthrough: bool) -> Self {
            self.tls_passthrough = tls_passthrough;
            self
        }
    }
    impl<S> From<(S, usize, S, usize)> for ForwardingRule
        where S: AsRef<str>
    {
        fn from(val: (S, usize, S, usize)) -> Self {
            ForwardingRule::new(val.0, val.1, val.2, val.3)
        }
    }
    impl<S> From<(S, usize, S, usize, Option<S>)> for ForwardingRule
        where S: AsRef<str>
    {
        fn from(val: (S, usize, S, usize, Option<S>)) -> Self {
            ForwardingRule::new(val.0, val.1, val.2, val.3).certificate_id(val.4)
        }
    }
    impl<S> From<(S, usize, S, usize, Option<S>, bool)> for ForwardingRule
        where S: AsRef<str>
    {
        fn from(val: (S, usize, S, usize, Option<S>, bool)) -> Self {
            ForwardingRule::new(val.0, val.1, val.2, val.3)
                .certificate_id(val.4)
                .tls_passthrough(val.5)
        }
    }

    /// This exists in the `health_check` field of a droplet.
    ///
    /// Health checks are used to tell if a Droplet is responding and should
    /// receive traffic. The Load Balancer will automatically stop sending
    /// traffic to unresponsive Droplets. You may specify the protocol, port,
    /// and path for a health check as well as additional setting such as the
    /// check interval and response timeout.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct HealthCheck {
        /// The protocol used for health checks sent to the backend Droplets.
        /// The possible values are "http" or "tcp".
        pub protocol: String,
        /// An integer representing the port on the backend Droplets on which
        /// the health check will attempt a connection.
        pub port: usize,
        /// The path on the backend Droplets to which the Load Balancer
        /// instance will send a request.
        pub path: String,
        /// The number of seconds between between two consecutive health
        /// checks.
        pub check_interval_seconds: usize,
        /// The number of seconds the Load Balancer instance will wait for a
        /// response until marking a health check as failed.
        pub response_timeout_seconds: usize,
        /// The number of times a health check must fail for a backend Droplet
        /// to be marked "unhealthy" and be removed from the pool.
        pub unhealthy_threshold: usize,
        /// The number of times a health check must pass for a backend Droplet
        /// to be marked "healthy" and be re-added to the pool.
        pub healthy_threshold: usize,
    }

    /// This exists in the `sticky_sessions` field of a droplet.
    ///
    /// When sticky sessions are in use, follow up requests from a client will
    /// be sent to the same Droplet as the original request. Both the name of
    /// the cookie and the TTL are configurable.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct StickySessions {
        /// An attribute indicating how and if requests from a client will be
        /// persistently served by the same backend Droplet. The possible
        /// values are "cookies" or "none".
        ///
        /// *Note:* Since `type` is a keyword in Rust `kind` is used instead.
        #[serde(rename = "type")]
        pub kind: String,
        /// The name of the cookie sent to the client. This attribute is only
        /// returned when using "cookies" for the sticky sessions type.
        pub cookie_name: Option<String>,
        /// The number of seconds until the cookie set by the Load Balancer
        /// expires. This attribute is only returned when using "cookies" for
        /// the sticky sessions type.
        pub cookie_ttl_seconds: Option<String>,
    }
}

impl LoadBalancer {
    /// Be sure to include a forwarding rule by chaining `.forwarding_rule()` onto this.
    ///
    /// **Note:** It may contain one of the droplets_ids or tag attributes as they are mutually exclusive.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-load-balancer)
    pub fn create<S>(name: S, region: S) -> Request<Create, LoadBalancer>
        where S: AsRef<str> + Serialize + Display
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(LOAD_BALANCERS_SEGMENT);

        Request::new(url).body(json!({
            "name": name,
            "region": region,
            "forwarding_rules": [],
        }))
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-load-balancer)
    pub fn get<S>(id: S) -> Request<Get, LoadBalancer>
        where S: AsRef<str> + Serialize + Display
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(LOAD_BALANCERS_SEGMENT)
            .push(id.as_ref());

        Request::new(url)
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-load-balancers)
    pub fn list() -> Request<List, Vec<LoadBalancer>> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(LOAD_BALANCERS_SEGMENT);

        Request::new(url)
    }
    /// **Note:** Any attribute that is not provided will be reset to its default value.
    ///
    /// **Note:** It may contain one of the droplets_ids or tag attributes as they are mutually exclusive.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#update-a-load-balancer)
    pub fn update<S>(id: S) -> Request<Update, LoadBalancer>
        where S: AsRef<str> + Serialize + Display
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(LOAD_BALANCERS_SEGMENT)
            .push(id.as_ref());

        Request::new(url)
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#delete-a-load-balancer)
    pub fn delete<S>(id: S) -> Request<Delete, ()>
        where S: AsRef<str> + Serialize + Display
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(LOAD_BALANCERS_SEGMENT)
            .push(id.as_ref());

        Request::new(url)
    }
}

impl Request<Create, LoadBalancer> {
    /// The load balancing algorithm used to determine which backend Droplet
    /// will be selected by a client. It must be either "round_robin" or
    /// "least_connections". The default value is "round_robin".
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-load-balancer)
    pub fn algorithm<S>(mut self, val: S) -> Request<Create, LoadBalancer>
        where S: Display + Serialize
    {
        self.body["algorithm"] = json!(val);
        self
    }
    /// An array of objects specifying the forwarding rules for a Load
    /// Balancer. At least one forwarding rule is required when creating a new
    /// Load Balancer instance.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-load-balancer)
    pub fn forwarding_rule<T>(mut self, val: T) -> Request<Create, LoadBalancer>
        where T: Into<ForwardingRule>
    {
        if !self.body["forwarding_rules"].is_array() {
            self.body["forwarding_rules"] = json!([]);
        }

        {
            let rules = self.body["forwarding_rules"]
                .as_array_mut()
                .expect("forwarding_rules should always be an array.");

            rules.push(json!(val.into()));
        }
        self
    }
    /// The (optional) health check settings.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-load-balancer)
    pub fn health_check<S>(mut self,
                           protocol: S,
                           port: usize,
                           path: Option<S>,
                           check_interval_seconds: Option<usize>,
                           response_timeout_seconds: Option<usize>,
                           unhealthy_threshold: Option<usize>,
                           healthy_threshold: Option<usize>)
                           -> Request<Create, LoadBalancer>
        where S: AsRef<str> + Display + Serialize
    {
        self.body["health_check"] = json!({
            "protocol": protocol,
            "port": port,
        });
        if let Some(path) = path {
            self.body["health_check"]["path"] = json!(path);
        }
        if let Some(check_interval_seconds) = check_interval_seconds {
            self.body["health_check"]["check_interval_seconds"] = json!(check_interval_seconds);
        }
        if let Some(response_timeout_seconds) = response_timeout_seconds {
            self.body["health_check"]["response_timeout_seconds"] = json!(response_timeout_seconds);
        }
        if let Some(unhealthy_threshold) = unhealthy_threshold {
            self.body["health_check"]["unhealthy_threshold"] = json!(unhealthy_threshold);
        }
        if let Some(healthy_threshold) = healthy_threshold {
            self.body["health_check"]["healthy_threshold"] = json!(healthy_threshold);
        }
        self
    }
    /// The (optional) sticky sessions settings. `kind` must be `cookies` or
    /// `none`. If `kind` is `cookies` then `cookie_name` and
    /// `cookie_ttl_seconds` should be set as well.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-load-balancer)
    pub fn sticky_sessions<S>(mut self,
                              kind: S,
                              cookie_name: Option<S>,
                              cookie_ttl_seconds: Option<usize>)
                              -> Request<Create, LoadBalancer>
        where S: AsRef<str> + Display + Serialize
    {
        self.body["sticky_sessions"] = json!({
            "type": kind,
        });
        if let Some(cookie_name) = cookie_name {
            self.body["sticky_sessions"]["cookie_name"] = json!(cookie_name);
        }
        if let Some(cookie_ttl_seconds) = cookie_ttl_seconds {
            self.body["sticky_sessions"]["cookie_ttl_seconds"] = json!(cookie_ttl_seconds);
        }
        self
    }
    /// A boolean value indicating whether HTTP requests to the Load Balancer
    /// on port 80 will be redirected to HTTPS on port 443. Default value is false.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-load-balancer)
    pub fn redirect_http_to_https(mut self, setting: bool) -> Request<Create, LoadBalancer> {
        self.body["redirect_http_to_https"] = json!(setting);
        self
    }
    /// The IDs of the Droplets to be assigned to the Load Balancer.
    ///
    /// **Note:** Not intended to be used alongside the `tag` function.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-load-balancer)
    pub fn droplets(mut self, ids: Vec<usize>) -> Request<Create, LoadBalancer> {
        self.body["droplet_ids"] = json!(ids);
        self
    }
    /// The name of a Droplet tag corresponding to Droplets to be assigned to
    /// the Load Balancer.
    ///
    /// **Note:** Not intended to be used alongside the `droplets` function.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-load-balancer)
    pub fn tag<S>(mut self, tag: S) -> Request<Create, LoadBalancer>
        where S: AsRef<str> + Display + Serialize
    {
        self.body["tag"] = json!(tag);
        self
    }
}


impl Request<Update, LoadBalancer> {
    /// A human-readable name for a Load Balancer instance.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#update-a-load-balancer)
    pub fn name<S>(mut self, val: S) -> Request<Update, LoadBalancer>
        where S: Display + Serialize
    {
        self.body["name"] = json!(val);
        self
    }
    /// The region where the Load Balancer instance will be located.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#update-a-load-balancer)
    pub fn region<S>(mut self, val: S) -> Request<Update, LoadBalancer>
        where S: Display + Serialize
    {
        self.body["region"] = json!(val);
        self
    }
    /// The load balancing algorithm used to determine which backend Droplet
    /// will be selected by a client. It must be either "round_robin" or
    /// "least_connections". The default value is "round_robin".
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#update-a-load-balancer)
    pub fn algorithm<S>(mut self, val: S) -> Request<Update, LoadBalancer>
        where S: Display + Serialize
    {
        self.body["algorithm"] = json!(val);
        self
    }
    /// An array of objects specifying the forwarding rules for a Load
    /// Balancer. At least one forwarding rule is required when creating a new
    /// Load Balancer instance.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#update-a-load-balancer)
    pub fn forwarding_rule<T>(mut self, val: T) -> Request<Update, LoadBalancer>
        where T: Into<ForwardingRule>
    {
        if !self.body["forwarding_rules"].is_array() {
            self.body["forwarding_rules"] = json!([]);
        }

        {
            let rules = self.body["forwarding_rules"]
                .as_array_mut()
                .expect("forwarding_rules should always be an array.");

            rules.push(json!(val.into()));
        }
        self
    }
    /// The (optional) health check settings.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#update-a-load-balancer)
    pub fn health_check<S>(mut self,
                           protocol: S,
                           port: usize,
                           path: Option<S>,
                           check_interval_seconds: Option<usize>,
                           response_timeout_seconds: Option<usize>,
                           unhealthy_threshold: Option<usize>,
                           healthy_threshold: Option<usize>)
                           -> Request<Update, LoadBalancer>
        where S: AsRef<str> + Display + Serialize
    {
        self.body["health_check"] = json!({
            "protocol": protocol,
            "port": port,
        });
        if let Some(path) = path {
            self.body["health_check"]["path"] = json!(path);
        }
        if let Some(check_interval_seconds) = check_interval_seconds {
            self.body["health_check"]["check_interval_seconds"] = json!(check_interval_seconds);
        }
        if let Some(response_timeout_seconds) = response_timeout_seconds {
            self.body["health_check"]["response_timeout_seconds"] = json!(response_timeout_seconds);
        }
        if let Some(unhealthy_threshold) = unhealthy_threshold {
            self.body["health_check"]["unhealthy_threshold"] = json!(unhealthy_threshold);
        }
        if let Some(healthy_threshold) = healthy_threshold {
            self.body["health_check"]["healthy_threshold"] = json!(healthy_threshold);
        }
        self
    }
    /// The (optional) sticky sessions settings. `kind` must be `cookies` or
    /// `none`. If `kind` is `cookies` then `cookie_name` and
    /// `cookie_ttl_seconds` should be set as well.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#update-a-load-balancer)
    pub fn sticky_sessions<S>(mut self,
                              kind: S,
                              cookie_name: Option<S>,
                              cookie_ttl_seconds: Option<usize>)
                              -> Request<Update, LoadBalancer>
        where S: AsRef<str> + Display + Serialize
    {
        self.body["sticky_sessions"] = json!({
            "type": kind,
        });
        if let Some(cookie_name) = cookie_name {
            self.body["sticky_sessions"]["cookie_name"] = json!(cookie_name);
        }
        if let Some(cookie_ttl_seconds) = cookie_ttl_seconds {
            self.body["sticky_sessions"]["cookie_ttl_seconds"] = json!(cookie_ttl_seconds);
        }
        self
    }
    /// A boolean value indicating whether HTTP requests to the Load Balancer
    /// on port 80 will be redirected to HTTPS on port 443. Default value is false.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#update-a-load-balancer)
    pub fn redirect_http_to_https(mut self, setting: bool) -> Request<Update, LoadBalancer> {
        self.body["redirect_http_to_https"] = json!(setting);
        self
    }
    /// The IDs of the Droplets to be assigned to the Load Balancer.
    ///
    /// **Note:** Not intended to be used alongside the `tag` function.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#update-a-load-balancer)
    pub fn droplets(mut self, ids: Vec<usize>) -> Request<Update, LoadBalancer> {
        self.body["droplet_ids"] = json!(ids);
        self
    }
    /// The name of a Droplet tag corresponding to Droplets to be assigned to
    /// the Load Balancer.
    ///
    /// **Note:** Not intended to be used alongside the `droplets` function.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#update-a-load-balancer)
    pub fn tag<S>(mut self, tag: S) -> Request<Update, LoadBalancer>
        where S: AsRef<str> + Display + Serialize
    {
        self.body["tag"] = json!(tag);
        self
    }
}

impl Request<Get, LoadBalancer> {
    /// Add droplets (by id) to the load balancer.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#add-droplets-to-a-load-balancer)
    pub fn add_droplets(mut self, ids: Vec<usize>) -> Request<Create, ()> {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLETS_SEGMENT);

        self.body = json!({
            "droplet_ids": ids,
        });

        self.method().value()
    }
    /// Remove droplets (by id) from the load balancer.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#remove-droplets-from-a-load-balancer)
    pub fn remove_droplets(mut self, ids: Vec<usize>) -> Request<Delete, ()> {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DROPLETS_SEGMENT);

        self.body = json!({
            "droplet_ids": ids,
        });

        self.method().value()
    }
    /// Add a forwarding rule to the Load Balancer.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#add-forwarding-rules-to-a-load-balancer)
    pub fn add_forwarding_rules<T>(mut self, items: Vec<T>) -> Request<Create, ()>
        where T: Into<ForwardingRule>
    {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(FORWARDING_RULES_SEGMENT);

        if !self.body["forwarding_rules"].is_array() {
            self.body["forwarding_rules"] = json!([]);
        }

        {
            let mut rules = self.body["forwarding_rules"]
                .as_array_mut()
                .expect("forwarding_rules should always be an array.");

            for item in items {
                let rule: ForwardingRule = item.into();
                rules.push(json!(rule));
            }
        }

        self.method().value()
    }
    /// Remove a forwarding rule to the Load Balancer.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#remove-forwarding-rules-from-a-load-balancer)
    pub fn remove_forwarding_rules<T>(mut self, items: Vec<T>) -> Request<Delete, ()>
        where T: Into<ForwardingRule>
    {
        self.url
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(FORWARDING_RULES_SEGMENT);

        if !self.body["forwarding_rules"].is_array() {
            self.body["forwarding_rules"] = json!([]);
        }

        {
            let mut rules = self.body["forwarding_rules"]
                .as_array_mut()
                .expect("forwarding_rules should always be an array.");

            for item in items {
                let rule: ForwardingRule = item.into();
                rules.push(json!(rule));
            }
        }

        self.method().value()
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LoadBalancerResponse {
    load_balancer: LoadBalancer,
}

impl HasResponse for LoadBalancer {
    type Response = LoadBalancerResponse;
}

impl HasValue for LoadBalancerResponse {
    type Value = LoadBalancer;
    fn value(self) -> LoadBalancer {
        self.load_balancer
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LoadBalancerListResponse {
    load_balancers: Vec<LoadBalancer>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasResponse for Vec<LoadBalancer> {
    type Response = LoadBalancerListResponse;
}

impl HasPagination for LoadBalancerListResponse {
    fn next_page(&self) -> Option<Url> {
        self.links.next()
    }
}

impl HasValue for LoadBalancerListResponse {
    type Value = Vec<LoadBalancer>;
    fn value(self) -> Vec<LoadBalancer> {
        self.load_balancers
    }
}
