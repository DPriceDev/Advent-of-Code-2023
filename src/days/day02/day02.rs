use crate::utils::file as file;

use regex::Regex;
use std::time::{Instant};
use once_cell::sync::{Lazy};

pub fn run() {
    println!("day 2");
    let now = Instant::now();
    println!("  part 1: {} in {}ms", part_one(), now.elapsed().as_millis());
    let now = Instant::now();
    println!("  part 2: {} in {}ms", part_two(), now.elapsed().as_millis());
}

fn part_one() -> u32 {
    let games = file::get_lines("input/day02/input.txt");
    static GAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"Game (\d+): (.*)").unwrap());

    games.iter().fold(0, |total, game| {
        let (_, [index, groups]) = GAME_REGEX.captures_iter(game.as_str())
            .next()
            .unwrap()
            .extract();

        let min_max = get_min_max(groups);
        if min_max.red <= 12 && min_max.green <= 13 && min_max.blue <= 14 {
            total + index.parse::<u32>().unwrap()
        } else {
            total
        }
    })
}

fn part_two() -> u32 {
    let games = file::get_lines("input/day02/input.txt");
    static GAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"Game (\d+): (.*)").unwrap());

    games.iter().fold(0, |total, game| {
        let (_, [_, groups]) = GAME_REGEX.captures_iter(game.as_str())
            .next()
            .unwrap()
            .extract();

        let min_max = get_min_max(groups);
        total + min_max.red * min_max.green * min_max.blue
    })
}

struct MaxSet {
    red: u32,
    green: u32,
    blue: u32,
}

fn get_min_max(groups: &str) -> MaxSet {
    static GREEN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+) green").unwrap());
    static RED_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+) red").unwrap());
    static BLUE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+) blue").unwrap());

    let mut max_set = MaxSet { red: 0, green: 0, blue: 0 };

    for group in groups.split(';').collect::<Vec<&str>>() {
        for capture in RED_REGEX.captures_iter(group) {
            let (_, [reds]) = capture.extract();
            let red_count = reds.parse::<u32>().unwrap();

            if max_set.red < red_count {
                max_set.red = red_count;
            }
        }

        for capture in GREEN_REGEX.captures_iter(group) {
            let (_, [greens]) = capture.extract();
            let green_count = greens.parse::<u32>().unwrap();

            if max_set.green < green_count {
                max_set.green = green_count;
            }
        }

        for capture in BLUE_REGEX.captures_iter(group) {
            let (_, [blues]) = capture.extract();
            let blue_count = blues.parse::<u32>().unwrap();

            if max_set.blue < blue_count {
                max_set.blue = blue_count;
            }
        }
    }

    return max_set;
}