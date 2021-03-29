use std::fs::File;
use std::io::Read;

pub struct Parser {
    contents: Vec<String>,
}

impl Parser {
    pub fn new(path: String) -> Self {
        let mut f = File::open(path).expect("file not found!");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        self::formatter(contents)
    }

    fn formatter(contents: String) -> Vec<String> {
        let mut iter = contents.split_whitespace();
        for s in iter {
            println!("{}", s);
        }
        Vec::<String>::new()
    }
}
