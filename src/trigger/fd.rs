use trigger::iface::Trigger;

use std::io::{Result, Error, ErrorKind, Read, BufRead, BufReader, Lines};

pub struct FdTrigger<A: Read> {
    fd: Lines<BufReader<A>>
}

impl<A: Read> FdTrigger<A> {
    pub fn new(fd: A) -> FdTrigger<A> {
        FdTrigger {
            fd: BufReader::new(fd).lines()
        }
    }
}

impl<A: Read> Trigger for FdTrigger<A> {
    fn available() -> bool {
        true
    }

    fn wait_for_next(&mut self) -> Result<()> {
        self.fd.next().unwrap_or(Err(Error::new(ErrorKind::BrokenPipe, "End of file"))).map(|_|{})
    }
}

impl<A: Read> Drop for FdTrigger<A> {
    fn drop(&mut self) {
    }
}
