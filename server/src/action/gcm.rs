extern crate jsonway;
extern crate serde_json;
extern crate regex;

use action::iface::Action;
use retry::retry_until;
use push_target_manager::PushTargetManager;

use std::io::Result;

use hyper::header;
use hyper::client::Client;
use hyper::status::StatusCode;
use self::serde_json::ser;
use self::regex::Regex;

const GCM_KEY: &'static str = include_str!("../../gcm_key.txt");
//const GCM_ENDPOINT: &'static str = "https://gcm-http.googleapis.com/gcm/send/fsc3M87LMQo:APA91bH15EC140SxXruImHDFrr-7RDJQyvHow8_Zlxq7OiFZoE9tYNxtfX2hXCrhCsIp8KoJhz9HwWojSo3aGkfn7lUaRXuWf4Y9gcKM0jv-HZ7B4vUEsasZrXWmBoZ3GXE_z2fEnOm1";
const GCM_ENDPOINT_REGEX: &'static str = r"^(?P<url>https://android\.googleapis\.com/gcm/send)/(?P<token>.*)$";

pub struct GcmAction {
    client: Client,
    gcm_endpoint_regex: Regex,
    targets: PushTargetManager
}

impl GcmAction {
    pub fn new(targets: &PushTargetManager) -> GcmAction {
        GcmAction {
            client: Client::new(),
            gcm_endpoint_regex: Regex::new(GCM_ENDPOINT_REGEX).unwrap(),
            targets: targets.clone()
        }
    }
}

impl GcmAction {
    fn parse_endpoint(&self, endpoint: &str) -> (String, Option<String>) {
        let gcm_captures = self.gcm_endpoint_regex.captures(endpoint);
        if let Some(gcm_captures) = gcm_captures {
            let url = gcm_captures.name("url").unwrap().replace("https://", "http://");
            let token = gcm_captures.name("token").map(|x| x.to_owned());
            println!("{} {:?}", url, token);
            (url, token)
        } else {
            (endpoint.to_owned(), None)
        }
    }

    fn send_to_endpoint(&mut self, endpoint: &str) -> Result<()> {
        let (url, token) = self.parse_endpoint(endpoint);
        let msg = format!("{}\r\n", ser::to_string(&jsonway::object(|json| {
            if let Some(ref x) = token {
                json.set("to", x);
            }
        }).unwrap()).unwrap());
        println!("{}: {}", msg.len(), msg);

        let result = retry_until(|| {
            self.client
                .post(&url)
                .body(&msg)
                .header(header::Authorization(format!("key={}", GCM_KEY)))
                .header(header::ContentType(mime!(Application/Json)))
                .header(header::ContentLength(msg.len() as u64))
                .send()
        }, |x| match x {
            &Ok(ref response) => {
                println!("{:?}", response);
                match response.status {
                    // For some reason, Google seems to return these spuriously...
                    StatusCode::LengthRequired => false,
                    StatusCode::NotFound => false,
                    _ => true
                }
            },
            _ => true
        }, 10);
        println!("{:?}", result);
        Ok(())
    }
}

impl Action for GcmAction {
    fn trigger(&mut self) -> Result<()> {
        for endpoint in self.targets.all() {
            try!(self.send_to_endpoint(&endpoint));
        }
        Ok(())
    }
}
