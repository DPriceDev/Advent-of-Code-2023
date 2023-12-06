use crate::utils::file as file;

use regex::Regex;
use std::time::{Instant};
use once_cell::sync::{Lazy};

pub fn run() {
    println!("day 6");
    let now = Instant::now();
    println!("  part 1: {} in {}us", part_one(), now.elapsed().as_micros());
    let now = Instant::now();
    println!("  part 2: {} in {}us", part_two(), now.elapsed().as_micros());
}

fn part_one() -> usize {
    static INPUT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
    file::get_lines("input/day06/input.txt").iter()
        .map(|line| INPUT_REGEX.find_iter(line))
        .map(|matches| {
            matches.map(|number| number.as_str().to_string().parse::<f64>().unwrap())
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>()
        .chunks(2)
        .map(|values| values.iter().nth(0).unwrap().iter().zip(values.iter().nth(1).unwrap()))
        .fold(1, |running, values| {
            running * values.fold(1, |running, (time, record)| running * calculate_winning_distance(*time, *record))
        })
}

fn part_two() -> usize {
    static INPUT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
    file::get_lines("input/day06/input.txt").iter()
        .map(|line| INPUT_REGEX.find_iter(line))
        .map(|matches| {
            matches.map(|number| number.as_str().to_string())
                .collect::<Vec<String>>()
                .join("")
                .parse::<f64>()
                .unwrap()
        })
        .collect::<Vec<f64>>()
        .chunks(2)
        .fold(0, |_, values| {
            calculate_winning_distance(*values.iter().nth(0).unwrap(), *values.iter().nth(1).unwrap())
        })
}

fn calculate_winning_distance(time: f64, record: f64) -> usize {
    let threshold = (time -((time * time) - (4f64 * record + 1f64)).sqrt()) / 2f64;
    ((time - threshold).ceil() - threshold).floor() as usize
}