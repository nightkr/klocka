use hyper::server::{Server, Request, Response, Handler};
use hyper::uri::RequestUri;
use hyper::mime::Mime;
use hyper::header::ContentType;
use hyper::status::StatusCode;
use hyper::Result;
use hyper;

use std::io::Read;

use push_target_manager::PushTargetManager;

const CONTENT_INDEX: &'static [u8] = include_bytes!("../../web-client/index.html");
const CONTENT_MANIFEST: &'static [u8] = include_bytes!("../../web-client/manifest.json");
const CONTENT_MAIN_JS: &'static [u8] = include_bytes!("../../web-client/main.js");
const CONTENT_WORKER_JS: &'static [u8] = include_bytes!("../../web-client/worker.js");
const CONTENT_404: &'static [u8] = include_bytes!("../../web-client/404");

fn write_static(content: &[u8], mime: Mime, mut res: Response) {
    {
        let mut headers = res.headers_mut();
        headers.set(ContentType(mime));
    }
    res.send(content).unwrap();
}

fn submit_endpoint(push_targets: &PushTargetManager, mut req: Request) {
    let mut endpoint = String::new();
    req.read_to_string(&mut endpoint).unwrap();
    push_targets.add(endpoint.as_ref());
    println!("Registered {}", endpoint)
}

struct WebHandler {
    push_targets: PushTargetManager
}

impl Handler for WebHandler {
    fn handle(&self, req: Request, mut res: Response) {
        let path = get_request_path(&req.uri);
        match (&req.method, path.as_ref().map(|x| x.as_ref())) {
            (&hyper::Get, Some("/")) => write_static(CONTENT_INDEX, mime!(Text/Html), res),
            (&hyper::Get, Some("/manifest.json")) => write_static(CONTENT_MANIFEST, mime!(Application/Json), res),
            (&hyper::Get, Some("/main.js")) => write_static(CONTENT_MAIN_JS, mime!(Application/Javascript), res),
            (&hyper::Get, Some("/worker.js")) => write_static(CONTENT_WORKER_JS, mime!(Application/Javascript), res),
            (&hyper::Post, Some("/submit")) => submit_endpoint(&self.push_targets, req),
            _ => {
                *res.status_mut() = StatusCode::NotFound;
                res.send(CONTENT_404).unwrap();
            }
        };
    }
}

fn get_request_path<'a>(uri: &RequestUri) -> Option<String> {
    match uri.clone() {
        RequestUri::AbsolutePath(full_path) => {
            full_path.split("?").next().map(|x| x.to_owned())
        },
        _ => None
    }
}

pub fn launch(push_targets: PushTargetManager, port: u16) -> Result<()> {
    let addr = ("0.0.0.0", port);
    let server = try!(Server::http(addr));
    try!(server.handle(WebHandler {
        push_targets: push_targets
    }));
    Ok(())
}
