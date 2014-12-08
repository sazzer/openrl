extern crate ncrs;

fn main() {
    let mut ncrs = ncrs::NCRS::new();
    println!("NCRS UI is {}x{} in size", ncrs.width(), ncrs.height());
}