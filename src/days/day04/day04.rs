use std::collections::HashSet;
use crate::utils::file as file;

use regex::Regex;
use std::time::{Instant};
use once_cell::sync::{Lazy};

pub fn run() {
    println!("day 4");
    let now = Instant::now();
    println!("  part 1: {} in {}ms", part_one(), now.elapsed().as_millis());
    let now = Instant::now();
    println!("  part 2: {} in {}ms", part_two(), now.elapsed().as_millis());
}

fn part_one() -> u32 {
    file::get_lines("input/day04/input.txt").iter()
        .map(get_wins)
        .sum::<u32>()
}

fn part_two() -> u32 {
    let input = file::get_lines("input/day04/example.txt");



    0
}

fn get_wins(input: &String) -> u32 {
    static INPUT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"([\d ]+)").unwrap());

    let (_, [winning_numbers]) = INPUT_REGEX.captures_iter(&input).nth(1).unwrap().extract();
    let (_, [our_numbers]) = INPUT_REGEX.captures_iter(&input).nth(2).unwrap().extract();

    let intersect_count = get_number_set(winning_numbers)
        .intersection(&get_number_set(our_numbers))
        .count();

    if intersect_count > 0 { 2_u32.pow((intersect_count - 1) as u32) } else { 0 }
}

fn get_number_set(input: &str) -> HashSet<u32> {
    static SPLIT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)").unwrap());
    SPLIT_REGEX.find_iter(&input)
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .collect::<HashSet<u32>>()
}