mod jack_analyzer;
mod jack_tokenizer;
mod compilation_engine;

use std::{env, ops::{Deref, DerefMut}};
use std::path::PathBuf;
fn main() {
    let args: Vec<String> = env::args().collect();
    jack_analyzer::analyze(&args[1]);
}



struct MyItem<'a> {
    bar_val: Vec<String>,
    bar_ref: &'a String,
}

impl<'a> MyItem<'a> {
    fn new(bar_val: Vec<String>) -> Self {
        MyItem {
            bar_val,
            bar_ref: &bar_val,
        }
    }
}
