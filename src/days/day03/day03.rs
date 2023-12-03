use std::collections::HashMap;
use crate::utils::file as file;

use regex::Regex;
use std::time::{Instant};
use once_cell::sync::{Lazy};

pub fn run() {
    println!("day 3");
    let now = Instant::now();
    println!("  part 1: {} in {}ms", part_one(), now.elapsed().as_millis());
    let now = Instant::now();
    println!("  part 2: {} in {}ms", part_two(), now.elapsed().as_millis());
}

fn part_one() -> u32 {
    let input_grid = file::get_lines("input/day03/input.txt");

    //let state = [mut [mut 0u8, ..4], ..4];

    let mut id = 0;
    let mut numbers = HashMap::new();
    let mut symbols: Vec<((usize, usize), char)> = Vec::new();

    for (row_index, row) in input_grid.iter().enumerate() {

        let mut index = 0;
        while index != row.len() {
            let entry = row.chars().nth(index).unwrap();

            if entry.is_digit(10) {
                let (number, length) = parse_number(&row[index..row.len()]);

                if number.is_some() {
                    for loop_index in 0..length {
                        let key = (index + loop_index, row_index);
                        numbers.insert( key, (number.unwrap(), id));
                    }
                    id += 1;

                    index += length - 1;
                }
            } else if entry != '.' {
                symbols.push(((index, row_index), entry))
            }

            index += 1;
        }

    }

    let mut total = 0;
    for ((x, y), symbol) in symbols {
        println!("checking {} at {}, {}", symbol, x, y);
        let to_check = [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];

        let mut ids = Vec::new();
        for (check_x, check_y) in to_check {
            let key = (check_x, check_y);
            let value = numbers.get(&key);
            if value.is_some() {
                let (number, id) = value.unwrap();
                if !ids.contains(&id) {
                    println!("    {} {} is {}", check_x, check_y, number);
                    total += number;
                    ids.push(id);
                }
            }
        }
    }

    total
}

fn parse_number(row: &str) -> (Option<u32>, usize) {
    let mut index = 0;
    while index != row.len() {
        if !row.chars().nth(index).unwrap().is_digit(10) {
            return (Some(row[0..index].parse::<u32>().unwrap()), index)
        }
        index += 1;
    }

    return (Some(row.parse::<u32>().unwrap()), row.len());
}

fn part_two() -> u32 {
    let games = file::get_lines("input/day03/input.txt");
    0
}
