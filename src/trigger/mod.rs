pub mod iface;
mod gpio;
mod fd;

pub use trigger::iface::Trigger;
use std::io::{Result, Error, ErrorKind, stdin};
use std::env;

pub fn create_trigger() -> Result<Box<Trigger>> {
    match env::var("KLOCKA_TRIGGER").unwrap_or("GPIO".to_string()).as_ref() {
        "FD" => {
            let trigger = fd::FdTrigger::new(stdin());
            Ok(Box::new(trigger))
        },
        "GPIO" => {
            let trigger = try!(gpio::GpioTrigger::new(22));
            Ok(Box::new(trigger))
        },
        x => {
            Err(Error::new(ErrorKind::NotFound, format!("Invalid KLOCKA_TRIGGER value {}, must be FD or GPIO", x)))
        }
    }
}
