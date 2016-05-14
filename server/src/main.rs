extern crate libc;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate mime;

mod polling;
mod retry;
mod push_target_manager;

mod action;
mod trigger;

use trigger::{create_trigger, Trigger};
use action::{create_actions, Action};
use push_target_manager::PushTargetManager;

fn main() {
    let push_targets = PushTargetManager::new();

    let mut trigger = create_trigger().unwrap();
    let mut actions = create_actions(&push_targets);

    loop {
        trigger.wait_for_next().unwrap();
        for action in actions.iter_mut() {
            action.trigger().unwrap();
        }
    }
}
