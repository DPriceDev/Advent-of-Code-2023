use std::time::Instant;
use crate::utils::file as file;

type Parser = fn(&str) -> Option<u32>;

pub fn run() {
    println!("day 1");
    let now = Instant::now();
    println!("  part 1: {} in {}ms", part_one(), now.elapsed().as_millis());
    let now = Instant::now();
    println!("  part 2: {} in {}ms", part_two(), now.elapsed().as_millis());
}

fn part_one() -> u32 {
    let parsers: [Parser; 1] = [
        |line| line.chars().next().unwrap().to_digit(10)
    ];
    parse_lines("input/day01/input.txt", parsers.as_slice())
}

fn part_two() -> u32 {
    let parsers: [Parser; 10] = [
        |line| line.chars().next().unwrap().to_digit(10),
        |line| match_word(line, ["one", "eno"].as_slice(), 1),
        |line| match_word(line, ["two", "owt"].as_slice(), 2),
        |line| match_word(line, ["three", "eerht"].as_slice(), 3),
        |line| match_word(line, ["four", "ruof"].as_slice(), 4),
        |line| match_word(line, ["five", "evif"].as_slice(), 5),
        |line| match_word(line, ["six", "xis"].as_slice(), 6),
        |line| match_word(line, ["seven", "neves"].as_slice(), 7),
        |line| match_word(line, ["eight", "thgie"].as_slice(), 8),
        |line| match_word(line, ["nine", "enin"].as_slice(), 9),
    ];
    parse_lines("input/day01/input.txt", parsers.as_slice())
}

fn match_word(line: &str, words: &[&str], value: u32) -> Option<u32> {
    if words.iter().any(|word| line.starts_with(word)) { Some(value) } else { None }
}

fn parse_lines(filename: &str, parsers: &[Parser]) -> u32 {
    file::get_lines(filename)
        .iter()
        .fold(0, |total, line| total + parse_line(line, parsers))
}

fn parse_line(line: &str, parsers: &[Parser]) -> u32 {
    let first_digit = find_number(line, parsers);
    let last_digit = find_number(line.chars().rev().collect::<String>().as_str(), parsers);

    format!("{}{}", first_digit, last_digit).parse::<u32>().unwrap()
}

fn find_number(line: &str, parsers: &[Parser]) -> u32 {
    for index in 0..line.len() {
        let window = &line[index..line.len()];
        for parser in parsers {
            let result = parser(window);
            if result.is_some() { return result.unwrap(); }
        }
    }
    panic!("number not found!")
}