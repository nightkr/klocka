extern crate libc;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate mime;

mod polling;
mod retry;
mod push_target_manager;
mod web_server;

mod action;
mod trigger;

use trigger::{create_trigger, Trigger};
use action::{create_actions, Action};
use push_target_manager::PushTargetManager;

use std::thread;

const WEB_PORT: u16 = 8080;

fn main() {
    let push_targets = PushTargetManager::new();

    let web_push_targets = push_targets.clone();
    thread::spawn(|| {
        web_server::launch(web_push_targets, WEB_PORT).unwrap()
    });

    let mut trigger = create_trigger().unwrap();
    let mut actions = create_actions(&push_targets);

    loop {
        trigger.wait_for_next().unwrap();
        for action in actions.iter_mut() {
            action.trigger().unwrap();
        }
    }
}
