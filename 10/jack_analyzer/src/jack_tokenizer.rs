use regex::Regex;
pub struct JackTokenizer {
    contents: Vec<String>,
    idx: usize,
}



impl JackTokenizer {
    pub fn new(contents: String) -> Self {
        let comment1 = Regex::new(r"//.*\n").unwrap();
        let comment2 = Regex::new(r"/*.**/").unwrap();

        let contents = comment1.replace_all(&contents, "");
        let contents = comment2.replace_all(&contents, "");
        let lines: Vec<String> = contents.split_whitespace()
            .map(|x| x.to_string()).collect();
        
        println!("{:?}", lines);

        JackTokenizer{
            contents: lines,
            idx: 0,
        }
    }

}