use std::{fs::{self, File}, io::{BufWriter, Write}, path:: PathBuf};
use crate::{compilation_engine::CompilationEngine, jack_tokenizer::{JackTokenizer, Tokens}};


pub fn analyze(args: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_pathes = VecFilePath::new(args);
    for i in 0..file_pathes.input_files.len() {
        let content = fs::read_to_string(&file_pathes.input_files[i])?;

        let tokenizer = JackTokenizer::new(content);
        let mut tokens = Tokens::new(tokenizer);
        let mut compilation_engine = CompilationEngine::new();
        let mut output = compilation_engine.compile(&mut tokens);

        // let vec_xml = tokens.get_xml();
        let vec_xml = output;

        let mut buf_writer = 
            BufWriter::new(File::create(&file_pathes.output_files[i]).unwrap());
        let xml = vec_xml.join("\n");
        write!(buf_writer, "{}", xml).unwrap();
        buf_writer.flush().unwrap();

    }
    Ok(())
}

#[derive(Debug)]
struct VecFilePath {
    input_files: Vec<PathBuf>,
    output_files: Vec<PathBuf>,
}
impl VecFilePath {
    fn new(args: &str) -> Self {
        let mut input_files = Vec::new();
        let mut output_files = Vec::new();

        let mut path = PathBuf::from(args);
        if let Some(file_name) = &path.file_name() {
            if file_name.to_str().unwrap().contains(".jack") {
                input_files.push(path.clone());
                path.set_extension("xml");
                output_files.push(path);
            } else {
                for dir_entry in fs::read_dir(path).unwrap() {
                    if let Ok(d) = dir_entry {
                        let mut path = d.path();
                        if path.to_str().unwrap().contains(".jack") {
                            input_files.push(path.clone());
                            path.set_extension("xml");
                            output_files.push(path);
                        }
                    }
                }
            }
        }
        VecFilePath {
            input_files,
            output_files,
        }
    }
}

#[test]
fn test() {
    let path1 = "./testcase/ArrayTest/Main.jack";
    let path2 = "./testcase/Square";
    println!("{:?}", VecFilePath::new(path1));
    println!("{:?}", VecFilePath::new(path2));
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
