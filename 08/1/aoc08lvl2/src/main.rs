// remember to change the module name!
use aoc08lvl2::Config;
use std::process;
fn main() {
    // remember to change the module name!
    if let Err(e) = aoc08lvl2::run(Config::make()) {
        eprintln!("{}", e);
        process::exit(1);
    } else {
        process::exit(0);
    }
}
