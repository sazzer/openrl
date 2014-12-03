#![feature(phase)]
#[phase(plugin, link)] extern crate log;

extern crate openrl_ui;

#[cfg(not(test))]
fn main() {
    info!("Output: {}", openrl_ui::openrl());
}
