use super::floating_ip::FloatingIp;
use super::Action;
use crate::method::{Create, Get, List};
use crate::request::{FloatingIpActionRequest, FloatingIpRequest};
use crate::STATIC_URL_ERROR;

const FLOATING_IP_ACTIONS_SEGMENT: &str = "actions";

impl FloatingIpRequest<Get, FloatingIp> {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-actions-for-a-floating-ip)
    pub fn actions(mut self) -> FloatingIpActionRequest<List, Vec<Action>> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(FLOATING_IP_ACTIONS_SEGMENT);

        self.transmute()
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-floating-ip-action)
    pub fn action(mut self, id: usize) -> FloatingIpActionRequest<Get, Action> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(FLOATING_IP_ACTIONS_SEGMENT)
            .push(&id.to_string());

        self.transmute()
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#unassign-a-floating-ip)
    pub fn unassign(mut self) -> FloatingIpActionRequest<Create, Action> {
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
    pub fn assign(mut self, id: usize) -> FloatingIpActionRequest<Create, Action> {
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
