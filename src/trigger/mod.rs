pub mod iface;
mod gpio;
mod fd;

pub use trigger::iface::Trigger;
use std::io::Result;

pub fn create_trigger() -> Result<Box<Trigger>> {
    let mut trigger = try!(gpio::GpioTrigger::new(22));
    Ok(Box::new(trigger))
}
