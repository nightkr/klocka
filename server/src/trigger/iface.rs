use std::io::Result;

pub trait Trigger : Drop {
    fn available() -> bool where Self: Sized;
    fn wait_for_next(&mut self) -> Result<()>;
}
