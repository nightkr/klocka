extern crate libc;

mod polling;
mod trigger;

use trigger::{create_trigger, Trigger};

fn main() {
    let mut trigger = create_trigger().unwrap();
    loop {
        trigger.wait_for_next().unwrap();
        println!("Triggered!")
    }
}
