pub mod iface;
mod gcm;
mod fd;

pub use action::iface::Action;
use push_target_manager::PushTargetManager;
use std::io::stdout;

pub fn create_actions(push_targets: &PushTargetManager) -> Vec<Box<Action>> {
    vec![
        Box::new(fd::FdAction::new(stdout())),
        Box::new(gcm::GcmAction::new(push_targets))
    ]
}
