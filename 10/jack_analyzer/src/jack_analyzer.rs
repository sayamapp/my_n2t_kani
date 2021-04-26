use std::fs;

pub fn analyze(args: &str) {
    let input_jack_pathes = read_args(args);
    

}

fn read_args(path: &str) -> Vec<String> {
    let mut res = Vec::new();

    if path.contains(".jack") {
        res.push(path.to_string());
    } else {
        match fs::read_dir(path) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(paths) => for path in paths {
                // let path = path.unwrap().file_name().to_str().unwrap().to_string();
                if let Ok(path) = path {
                    let path = path.file_name().to_str().unwrap().to_string();
                    if path.contains(".jack") {
                        res.push(path);
                    }
                }
            }
        }
    }
    res
}
