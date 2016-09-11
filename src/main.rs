
use std::env;

mod lib;
use lib::*;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let input = argv[1].clone();
    let output = boldline(input, Marking::ANSIBold, Pattern::Cross);
    println!("{}", output.join("\n"));
}
