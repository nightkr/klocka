extern crate hyper;
extern crate jsonway;
extern crate serde_json;

use action::iface::Action;

use std::io::Result;

use self::hyper::client::Client;
use self::serde_json::ser;

const GCM_KEY: &'static str = include_str!("../../gcm_key.txt");
const GCM_ENDPOINT: &'static str = "http://gcm-http.googleapis.com/gcm/send";
const GCM_TARGET: &'static str = "fsc3M87LMQo:APA91bH15EC140SxXruImHDFrr-7RDJQyvHow8_Zlxq7OiFZoE9tYNxtfX2hXCrhCsIp8KoJhz9HwWojSo3aGkfn7lUaRXuWf4Y9gcKM0jv-HZ7B4vUEsasZrXWmBoZ3GXE_z2fEnOm1";

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
        let msg = format!("{}\r\n", ser::to_string(&jsonway::object(|json| {
            json.object("notification", |json| {
                json.set("title", "Klocka");
                json.set("text", "Open the door!");
            });

            json.set("to", GCM_TARGET);
        }).unwrap()).unwrap());
        println!("{}: {}", msg.len(), msg);
        let request = self.client
            .post(GCM_ENDPOINT)
            .body(&msg)
            .header(hyper::header::Authorization(format!("key={}", GCM_KEY)))
            .header(hyper::header::ContentType(mime!(Application/Json)))
            .header(hyper::header::ContentLength(msg.len() as u64));
        let result = request
            .send()
            .unwrap();
        println!("{:?}", result);
        Ok(())
    }
}
