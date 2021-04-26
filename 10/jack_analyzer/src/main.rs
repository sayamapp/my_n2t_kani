mod jack_analyzer;
mod jack_tokenizer;

use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    jack_analyzer::analyze(&args[1]);
}

