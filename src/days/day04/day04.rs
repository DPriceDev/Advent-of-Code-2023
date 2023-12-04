use std::collections::{HashMap, HashSet};
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
        .map(get_win_count)
        .map(|count| if count > 0 { 2_u32.pow((count - 1) as u32) } else { 0 })
        .sum::<u32>()
}

fn part_two() -> u32 {
    let counts = file::get_lines("input/day04/input.txt").iter()
        .map(get_win_count)
        .collect::<Vec<usize>>();

    let mut card_counts: HashMap<usize, u32> = HashMap::new();
    for (index, count) in counts.iter().enumerate() {
        let additional_count = update_card_count(&mut card_counts, index, 1);

        for offset in 0..*count {
            update_card_count(&mut card_counts, index + offset + 1, additional_count);
        }
    }

    card_counts.values().sum::<u32>()
}

fn update_card_count(counts: &mut HashMap<usize, u32>, index: usize, additional_count: u32) -> u32 {
    let new_count = counts.get(&index).unwrap_or(&0) + additional_count;
    counts.insert(index, new_count);
    return new_count
}

fn get_win_count(input: &String) -> usize {
    static INPUT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"([\d ]+)").unwrap());

    let (_, [winning_numbers]) = INPUT_REGEX.captures_iter(&input).nth(1).unwrap().extract();
    let (_, [our_numbers]) = INPUT_REGEX.captures_iter(&input).nth(2).unwrap().extract();

    get_number_set(winning_numbers)
        .intersection(&get_number_set(our_numbers))
        .count()
}

fn get_number_set(input: &str) -> HashSet<u32> {
    static SPLIT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)").unwrap());
    SPLIT_REGEX.find_iter(&input)
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .collect::<HashSet<u32>>()
}