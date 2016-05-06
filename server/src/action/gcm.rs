extern crate hyper;
extern crate jsonway;
extern crate serde_json;

use action::iface::Action;

use std::io::Result;

use self::hyper::client::Client;
use self::serde_json::ser;

const GCM_KEY: &'static str = include_str!("../../gcm_key.txt");
const GCM_ENDPOINT: &'static str = "https://gcm-http.googleapis.com/gcm/send";
const GCM_TARGET: &'static str = "";

pub struct GcmAction {
    client: Client
}

impl GcmAction {
    pub fn new() -> GcmAction {
        GcmAction {
            client: Client::new()
        }
    }
}

impl Action for GcmAction {
    fn trigger(&mut self) -> Result<()> {
        let msg = ser::to_string(&jsonway::object(|json| {
            json.object("notification", |json| {
                json.set("title", "Klocka");
                json.set("text", "Open the door!");
            });

            json.set("to", GCM_TARGET);
        }).unwrap()).unwrap();
        let result = self.client
            .post(GCM_ENDPOINT)
            .body(&msg)
            .header(hyper::header::ContentType(mime!(Application/Json)))
            .header(hyper::header::Authorization(format!("key={}", GCM_KEY)))
            .send()
            .unwrap();
        println!("{:?}", result);
        Ok(())
    }
}
