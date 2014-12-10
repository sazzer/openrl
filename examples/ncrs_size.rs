extern crate ncrs;

fn main() {
    let ncrs = ncrs::NCRS::new();
    println!("NCRS UI is {}x{} in size", ncrs.width(), ncrs.height());
}