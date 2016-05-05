use trigger::iface::Trigger;
use std::path::{Path, PathBuf};
use std::io::{Result, Error, ErrorKind, Read, Write, Seek, SeekFrom};
use std::fs::{File, OpenOptions};
use polling::Poll;
use std::thread::sleep;
use std::time::Duration;
use std::fmt::Display;
use libc::{POLLPRI, POLLERR};

const BASE_PATH: &'static str = "/sys/class/gpio";
const ACTIVE_LOW: bool = true;

fn open_write() -> OpenOptions {
    let mut opts = OpenOptions::new();
    opts.write(true);
    opts.create(false);
    opts.truncate(false);
    opts
}

fn write_file<A: AsRef<Path>, B: Display>(path: A, msg: B) -> Result<()> {
    let mut file = try!(open_write().open(path));
    write!(file, "{}", msg)
}

pub struct GpioTrigger {
    pin: u8,
    path: PathBuf,
    value_fd: Option<File>
}

impl GpioTrigger {
    pub fn new(pin: u8) -> Result<GpioTrigger> {
        try!(GpioTrigger::assert_available());
        println!("available");

        let path = Path::new(BASE_PATH).join(format!("gpio{}", pin));
        let mut obj = GpioTrigger {
            pin: pin,
            path: path,
            value_fd: None
        };
        try!(obj.init());
        Ok(obj)
    }

    fn init(&mut self) -> Result<()> {
        try!(self.uninit());
        let base = Path::new(BASE_PATH);
        println!("exporting {}", self.pin);
        try!(write_file(base.join("export"), self.pin));
        println!("done");

        println!("path: {:?}", self.path);
        // Wait for the pin to become available
        // FIXME: Any way to wait for this dynamically instead?
        sleep(Duration::from_millis(100));

        println!("setting direction");
        try!(write_file(self.path.join("direction"), "in"));
        println!("setting active_low");
        try!(write_file(self.path.join("active_low"), if ACTIVE_LOW {1} else {0}));
        println!("setting edge");
        try!(write_file(self.path.join("edge"), "rising"));

        println!("opening");
        self.value_fd = Some(try!(File::open(self.path.join("value"))));

        println!("done!");
        Ok(())
    }

    fn uninit(&mut self) -> Result<()> {
        if !self.path.exists() {
            println!("pin {} not exported, skipping unexport...", self.pin);
            return Ok(())
        }

        let unexport_path = Path::new(BASE_PATH).join("unexport");
        let mut unexport = try!(open_write().open(unexport_path.as_path()));
        println!("unexporting {}", self.pin);
        try!(write!(unexport, "{}", self.pin));
        println!("done");
        Ok(())
    }

    fn assert_available() -> Result<()> {
        if GpioTrigger::available() {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::NotFound, format!("GPIO is not available ({} was not found)", BASE_PATH)))
        }
    }
}

impl Trigger for GpioTrigger {
    fn available() -> bool {
        Path::new(BASE_PATH).exists()
    }

    fn wait_for_next(&mut self) -> Result<()> {
        self.value_fd.as_mut().map_or(Ok(()), |fd| {
            try!(fd.seek(SeekFrom::Start(0)));
            let mut buf = vec![0];
            try!(fd.read(&mut buf));
            fd.poll(POLLPRI | POLLERR, -1).map(|_| {})
        })
    }
}

impl Drop for GpioTrigger {
    fn drop(&mut self) {
        self.uninit().unwrap();
    }
}
