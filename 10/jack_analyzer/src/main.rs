mod jack_analyzer;
mod jack_tokenizer;

use std::{env, fs, path::PathBuf};
fn main() {
    let args: Vec<String> = env::args().collect();
    jack_analyzer::analyze(&args[1]);
}

#[test]
fn test() {
    let path1 = "./testcase/ArrayTest/Main.jack";
    let path2 = "./testcase/Square/";
    let path3 = "./testcase/Square";

    let mut buf = PathBuf::from(path3);

    let mut input_files = Vec::new();
    let mut output_files = Vec::new();

    if !buf.file_name().unwrap().to_str().unwrap().contains(".jack") {
        let file_pathes = fs::read_dir(buf).unwrap();
        for dir_entry in file_pathes {
            if let Ok(d) = dir_entry {
                if d.path().to_str().unwrap().contains(".jack") {
                    let input_path = d.path().clone();
                    let mut output_path = d.path();
                    output_path.set_file_name(format!(
                        "{}{}",
                        output_path.file_stem().unwrap().to_str().unwrap(),
                        ".xml"
                    ));
                    input_files.push(input_path);
                    output_files.push(output_path);
                }
            }
        }
    }

    println!("{:?}", input_files);
    println!("{:?}", output_files);
}
