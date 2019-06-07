use pseudolocalize::transform::transform_str;
use std::env::args;

fn main() {
    for arg in args() {
        println!("{}", transform_str(arg.as_str()));
    }
}