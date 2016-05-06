pub mod iface;
mod gcm;
mod fd;

pub use action::iface::Action;
use std::io::stdout;

pub fn create_actions() -> Vec<Box<Action>> {
    vec![
        Box::new(fd::FdAction::new(stdout())),
        Box::new(gcm::GcmAction::new())
    ]
}
