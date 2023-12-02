use std::fs;

pub fn get_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap().lines()
        .map(String::from)
        .collect()
}