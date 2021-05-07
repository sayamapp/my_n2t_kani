mod jack_tokenizer;
mod compilation_engine;

use std::{env, thread, time::Duration};
use std::io::{BufWriter, Write};
use std::fs;
use std::fs::File;
use std::path::PathBuf;

use crate::jack_tokenizer::Tokenizer;
use crate::compilation_engine::CompilationEngine;

use indicatif::{ProgressBar, ProgressStyle};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // get jack files
    let jack_files = get_jack_files(&args[1]);
    let count = jack_files.len();

    // compile jack files
    for (i, jack_file) in jack_files.into_iter().enumerate() {

        let jack_file_name = jack_file.clone();
        let jack_file_name = jack_file_name.to_str().unwrap();
        let pb = ProgressBar::new(100);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos:>3}/{len:3} {msg}")
            .progress_chars("##-"));
    

        
        // construct jack tokenizer
        let tokenizer = Tokenizer::new(&jack_file);
        // println!("{:?}", tokenizer); //DEBUG

        let mut compilation_engine = CompilationEngine::new(tokenizer);
        compilation_engine.start_compile();

        let output_xml = compilation_engine.output_xml();
        write_xml(jack_file, &output_xml);


        for _ in 0..100 {
            pb.set_message(format!("[{}/{}] {} compile to xml ", i + 1, count, jack_file_name));
            pb.inc(1);
            thread::sleep(Duration::from_millis(5));
        }
        pb.finish();
    }
    println!("Compile to xml done!");

    Ok(())
}

fn get_jack_files(path: &str) -> Vec<PathBuf> {
    let input_pb = PathBuf::from(path);
    let mut output_pbs = Vec::new();
    if input_pb.is_file() {
        output_pbs.push(input_pb);
    } else {
        for d_entry in fs::read_dir(input_pb).unwrap() {
            if let Ok(file) = d_entry {
                let file_path = file.path();
                if file_path.extension().unwrap().to_str() == Some("jack") {
                    output_pbs.push(file_path);
                }
            }
        }
    }
    output_pbs
}

fn write_xml(mut path_buf: PathBuf, vec_xml: &Vec<String>) {
    path_buf.set_extension("xml");
    let mut buf_writer = BufWriter::new(File::create(&path_buf).unwrap());
    let xml = vec_xml.join("\n");
    write!(buf_writer, "{}", xml).unwrap();
    buf_writer.flush().unwrap();
}
