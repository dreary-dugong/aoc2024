// remember to change the module name!
use aoc07lvl1::Config;
use std::process;
fn main() {
    // remember to change the module name!
    if let Err(e) = aoc07lvl1::run(Config::make()) {
        eprintln!("{}", e);
        process::exit(1);
    } else {
        process::exit(0);
    }
}
