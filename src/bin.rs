#![feature(phase)]
#[phase(plugin, link)] extern crate log;

extern crate openrl_common;

#[cfg(not(test))]
fn main() {
    info!("Output: {}", openrl_common::openrl());
}
