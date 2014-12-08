extern crate ncrs;

fn main() {
    let mut ncrs = ncrs::NCRS::new();
    ncrs.option(ncrs::Options::Echo(false));
    ncrs.option(ncrs::Options::CBreak(ncrs::CBreakMode::Raw));
    ncrs.option(ncrs::Options::Keypad(true));
    ncrs.option(ncrs::Options::Meta(true));
    ncrs.option(ncrs::Options::QiFlush(false));
}