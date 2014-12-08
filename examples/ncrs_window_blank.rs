extern crate ncrs;

fn main() {
    let mut ncrs = ncrs::NCRS::new();
    let mut window = ncrs.new_window("example", ncrs::window::WindowOptions {x: 5, y: 5, width: 10, height: 10, ..std::default::Default::default()});
}