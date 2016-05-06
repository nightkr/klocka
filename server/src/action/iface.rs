use std::io::Result;

pub trait Action {
    fn trigger(&mut self) -> Result<()>;
}
