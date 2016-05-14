extern crate hyper;
extern crate jsonway;
extern crate serde_json;
extern crate regex;

use action::iface::Action;
use retry::retry_until;

use std::io::Result;

use self::hyper::client::Client;
use self::serde_json::ser;
use self::regex::Regex;

const GCM_KEY: &'static str = include_str!("../../gcm_key.txt");
//const GCM_ENDPOINT: &'static str = "https://gcm-http.googleapis.com/gcm/send/fsc3M87LMQo:APA91bH15EC140SxXruImHDFrr-7RDJQyvHow8_Zlxq7OiFZoE9tYNxtfX2hXCrhCsIp8KoJhz9HwWojSo3aGkfn7lUaRXuWf4Y9gcKM0jv-HZ7B4vUEsasZrXWmBoZ3GXE_z2fEnOm1";
const GCM_ENDPOINT: &'static str = "https://android.googleapis.com/gcm/send/d5d53ZcncjE:APA91bGFjvRTnORHtR_TYQqM0p7ld_xPmZcl0-NwBBpOFke5LRMgL_RR_-ZPSx9OXCuPbBUxlIrBOK3TFuBi163kzMFCtsEqCu0MkxDfQdOAXs2ii_U_OIpLI3IKYAVn1aFj2jMD964i";
const GCM_ENDPOINT_REGEX: &'static str = r"^(?P<url>https://android\.googleapis\.com/gcm/send)/(?P<token>.*)$";

pub struct GcmAction {
    client: Client,
    gcm_endpoint_regex: Regex
}

impl GcmAction {
    pub fn new() -> GcmAction {
        GcmAction {
            client: Client::new(),
            gcm_endpoint_regex: Regex::new(GCM_ENDPOINT_REGEX).unwrap()
        }
    }
}

impl GcmAction {
    fn parse_endpoint<'a>(&self, endpoint: &'a str) -> (&'a str, Option<&'a str>) {
        let gcm_captures = self.gcm_endpoint_regex.captures(endpoint);
        if let Some(gcm_captures) = gcm_captures {
            let url = gcm_captures.name("url").unwrap();
            let token = gcm_captures.name("token");
            println!("{} {:?}", url, token);
            (url, token)
        } else {
            (endpoint, None)
        }
    }
}

impl Action for GcmAction {
    fn trigger(&mut self) -> Result<()> {
        let (url, token) = self.parse_endpoint(GCM_ENDPOINT);
        let msg = format!("{}\r\n", ser::to_string(&jsonway::object(|json| {
            json.object("notification", |json| {
                json.set("title", "Klocka");
                json.set("text", "Open the door!");
            });

            if let Some(x) = token {
                json.set("to", x);
            }
        }).unwrap()).unwrap());
        println!("{}: {}", msg.len(), msg);

        let result = retry_until(|| {
            self.client
                .post(url)
                .body(&msg)
                .header(hyper::header::Authorization(format!("key={}", GCM_KEY)))
                .header(hyper::header::ContentType(mime!(Application/Json)))
                .header(hyper::header::ContentLength(msg.len() as u64))
                .send()
        }, |x| match x {
            &Ok(ref response) => {
                println!("{:?}", response);
                response.status == hyper::Ok
            },
            _ => false
        }, 10);
        println!("{:?}", result);
        Ok(())
    }
}
