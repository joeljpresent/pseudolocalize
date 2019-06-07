use pseudolocalize::pseudolocalizer::Pseudolocalizer;
use std::env::args;

fn main() {
    for arg in args() {
        let pl = Pseudolocalizer::new();
        println!("{}", pl.transform(arg.as_str()));
    }
}