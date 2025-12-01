use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn get_lines_from_file(str_path: &str) -> Vec<String> {
    let path = Path::new(str_path);
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => panic!("Couldn't open {}", path.display())
    };

    let reader = BufReader::new(file);
    let lines = reader.lines().map(|res| match res {
        Ok(string) => string,
        Err(_) => panic!("Couldn't parse lines from file")
    }).collect();
    lines
}
