use std::sync::{Arc, Mutex};
use std::fs::{File, OpenOptions};
use std::io::{Result, BufRead, BufReader, ErrorKind, Write};
use std::collections::HashSet;

const FILE_PATH: &'static str = "push_targets.txt";

#[derive(Clone)]
pub struct PushTargetManager {
    endpoints: Arc<Mutex<HashSet<String>>>
}

impl PushTargetManager {
    pub fn new() -> PushTargetManager {
        let mut ptm = PushTargetManager {
            endpoints: Arc::new(Mutex::new(HashSet::new()))
        };
        match ptm.read_from_file() {
            Err(ref e) if e.kind() == ErrorKind::NotFound => println!("{} not found, starting with no targets", FILE_PATH),
            x => x.unwrap()
        }
        ptm
    }

    fn read_from_file(&mut self) -> Result<()> {
        let mut endpoints = self.endpoints.lock().unwrap();
        let file = BufReader::new(try!(File::open(FILE_PATH)));
        for line in file.lines() {
            endpoints.insert(try!(line));
        }
        Ok(())
    }

    fn add_to_file(&self, endpoint: &str) -> Result<()> {
        let mut file = try!(OpenOptions::new()
            .append(true)
            .create(true)
            .open(FILE_PATH));
        writeln!(file, "{}", endpoint)
    }

    pub fn all(&self) -> HashSet<String> {
        let endpoints = self.endpoints.lock().unwrap();
        endpoints.clone()
    }

    pub fn add(&self, endpoint: &str) {
        let mut endpoints = self.endpoints.lock().unwrap();
        let is_new = endpoints.insert(endpoint.to_owned());
        if is_new {
            self.add_to_file(endpoint).unwrap()
        }
    }
}
