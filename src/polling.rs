use libc::{poll, pollfd, POLLPRI};
use std::io::{Result, Error};
use std::os::unix::io::AsRawFd;

pub trait Poll {
    fn poll(&mut self, event_mask: i16, timeout: i32) -> Result<i32>;
}

impl<A: AsRawFd> Poll for A {
    fn poll(&mut self, event_mask: i16, timeout: i32) -> Result<i32> {
        let fd = self.as_raw_fd();
        let mut built_pollfd = pollfd {
            fd: fd,
            events: event_mask,
            revents: 0
        };
        let changed_files = unsafe {
            poll(&mut built_pollfd, 1, timeout)
        };
        if changed_files < 0 {
            Err(Error::last_os_error())
        } else {
            println!("{}", changed_files);
            Ok(changed_files)
        }
    }
}
