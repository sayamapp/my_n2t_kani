mod jack_analyzer;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    jack_analyzer::analyze(&args[1]);
}

