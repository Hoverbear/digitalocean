use request::Request;
use method::{Create, Get, List};
use super::FloatingIp;
use STATIC_URL_ERROR;
use super::Action;

const FLOATING_IP_ACTIONS_SEGMENT: &'static str = "actions";

impl Request<Get, FloatingIp> {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-actions-for-a-floating-ip)
    pub fn actions(mut self) -> Request<List, Vec<Action>> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(FLOATING_IP_ACTIONS_SEGMENT);

        self.transmute()
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-floating-ip-action)
    pub fn action(mut self, id: usize) -> Request<Get, Action> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(FLOATING_IP_ACTIONS_SEGMENT)
            .push(&id.to_string());

        self.transmute()
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#unassign-a-floating-ip)
    pub fn unassign(mut self) -> Request<Create, Action> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(FLOATING_IP_ACTIONS_SEGMENT);

        self.set_body(json!({
            "type": "unassign",
        }));

        self.transmute()
    }


    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#assign-a-floating-ip-to-a-droplet)
    pub fn assign(mut self, id: usize) -> Request<Create, Action> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(FLOATING_IP_ACTIONS_SEGMENT);

        self.set_body(json!({
            "type": "assign",
            "droplet_id": id,
        }));

        self.transmute()
    }
}
