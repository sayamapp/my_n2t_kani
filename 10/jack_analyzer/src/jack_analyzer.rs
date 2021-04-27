use std::{char::ToLowercase, fs};
use crate::jack_tokenizer::JackTokenizer;

pub fn analyze(args: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input_jack_pathes = read_args(args);
    println!("{:?}", input_jack_pathes);
    for jack_filepath in input_jack_pathes {
        let content = fs::read_to_string(&jack_filepath)?;

        let tokenizer = JackTokenizer::new(content);

    }
    Ok(())
}

fn read_args(path: &str) -> Vec<String> {
    let mut res = Vec::new();

    if path.contains(".jack") {
        res.push(path.to_string());
    } else {
        let paths = fs::read_dir(path).unwrap();
        for path in paths {
            let path = path.unwrap().path().to_str().unwrap().to_string();
            if path.contains(".jack") {
                res.push(path);
            }
        }
    }
    res
}
