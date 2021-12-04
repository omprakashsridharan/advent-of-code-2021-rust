use std::fs::File;
use std::io::{self, BufRead};

pub fn get_lines(name: &str) -> Vec<String> {
    let file = File::open(format!("./{}.txt", name)).unwrap();
    let values: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| {
            if let Ok(line) = l {
                return line;
            } else {
                return String::from("");
            }
        })
        .collect();
    return values;
}
