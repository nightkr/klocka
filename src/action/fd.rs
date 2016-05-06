use action::iface::Action;

use std::io::{Result, Write};

pub struct FdAction<A: Write> {
    fd: A
}

impl<A: Write> FdAction<A> {
    pub fn new(fd: A) -> FdAction<A> {
        FdAction {
            fd: fd
        }
    }
}

impl<A: Write> Action for FdAction<A> {
    fn trigger(&mut self) -> Result<()> {
        writeln!(self.fd, "Triggered!")
    }
}
