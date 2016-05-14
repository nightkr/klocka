use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct PushTargetManager {
    endpoints: Arc<Mutex<Vec<String>>>
}

impl PushTargetManager {
    pub fn new() -> PushTargetManager {
        PushTargetManager {
            endpoints: Arc::new(Mutex::new(vec![
                "https://android.googleapis.com/gcm/send/d5d53ZcncjE:APA91bGFjvRTnORHtR_TYQqM0p7ld_xPmZcl0-NwBBpOFke5LRMgL_RR_-ZPSx9OXCuPbBUxlIrBOK3TFuBi163kzMFCtsEqCu0MkxDfQdOAXs2ii_U_OIpLI3IKYAVn1aFj2jMD964i".to_owned()
            ]))
        }
    }

    pub fn all(&self) -> Vec<String> {
        let endpoints = self.endpoints.lock().unwrap();
        endpoints.clone()
    }

    pub fn add(&self, endpoint: &str) {
        let mut endpoints = self.endpoints.lock().unwrap();
        endpoints.push(endpoint.to_owned());
    }
}
