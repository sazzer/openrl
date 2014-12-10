#![feature(phase)]
#[phase(plugin, link)] extern crate log;

extern crate ncrs;

fn main() {
    let mut ncrs = ncrs::NCRS::new();
    ncrs.new_window("example", ncrs::window::WindowOptions {x: 5, y: 5, width: 10, height: 10, ..std::default::Default::default()});
    
    let window_names = ncrs.get_window_names();
    for name in window_names.iter() {
        info!("Window named {} exists", name);
    }

    ncrs.render();
}