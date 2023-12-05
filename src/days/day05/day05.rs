use std::cmp::{max, min, Ordering};
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

struct Product {
    next: String,
    ranges: Vec<Range>,
}

struct Mapping {
    start: usize,
    dest_start: usize,
    range: usize,
}

struct Range {
    start: usize,
    end: usize,
}



fn part_one() -> usize {
    let almanac = file::get_lines("input/day05/input.txt");

    let mut seeds_part = get_seeds(almanac.clone().into_iter().nth(0).unwrap().as_str())
        .into_iter()
        .map(|seeds| Range { start: seeds, end: seeds })
        .collect::<Vec<Range>>();

    let mut row_index = 2;
    while row_index < almanac.len() {
        let (mappings, new_index) = get_mappings(row_index, &almanac);
        seeds_part = map_ranges(&seeds_part, &mappings);
        row_index = new_index
    }

    seeds_part.into_iter()
        .min_by(|a, b| a.start.cmp(&b.start))
        .unwrap()
        .start
}

fn part_two() -> usize {
    let almanac = file::get_lines("input/day05/input.txt");

    let seeds = get_seeds(almanac.clone().into_iter().nth(0).unwrap().as_str());

    let mut seeds_part = seeds.into_iter()
        .as_slice()
        .chunks(2)
        .map(|seeds| {
            Range {
                start: *seeds.into_iter().nth(0).unwrap(),
                end: *seeds.into_iter().nth(0).unwrap() + *seeds.into_iter().nth(1).unwrap()
            }
        })
        .collect::<Vec<Range>>();

    let mut row_index = 2;
    while row_index < almanac.len() {
        let (mappings, new_index) = get_mappings(row_index, &almanac);
        seeds_part = map_ranges(&seeds_part, &mappings);
        row_index = new_index
    }

    seeds_part.into_iter()
        .min_by(|a, b| a.start.cmp(&b.start))
        .unwrap()
        .start

    // let input = Vec::from([Range { start: 0, end: 12 }]);
    //
    // let mappings = Vec::from([
    //     Mapping {
    //         start: 2,
    //         dest_start: 40,
    //         range: 5,
    //     },
    //     Mapping {
    //         start: 10,
    //         dest_start: 50,
    //         range: 7,
    //     },
    //     Mapping {
    //         start: 50,
    //         dest_start: 100,
    //         range: 7,
    //     },
    // ]);
    //
    // let new_ranges = map_ranges(&input, &mappings);
    //
    // for new_range in new_ranges {
    //     println!("{} - {}", new_range.start, new_range.end)
    // }
}

fn get_seeds(input: &str) -> Vec<usize> {
    Regex::new(r"(\d+)").unwrap()
        .find_iter(input)
        .map(|seed| seed.as_str().parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn get_mappings(index: usize, almanac: &Vec<String>) -> (Vec<Mapping>, usize) {
    let mut row_index = index + 1;

    let mut list = Vec::new();
    // row_index += 1;
    static MAPPING_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
    while row_index < almanac.len() {
        let row = almanac.into_iter().nth(row_index).unwrap().as_str();

        if row.is_empty() {
            row_index += 1;
            return (list, row_index)
        }

        let destination_start = MAPPING_REGEX.find_iter(row).nth(0).unwrap().as_str().parse::<usize>().unwrap();
        let source_start = MAPPING_REGEX.find_iter(row).nth(1).unwrap().as_str().parse::<usize>().unwrap();
        let range = MAPPING_REGEX.find_iter(row).nth(2).unwrap().as_str().parse::<usize>().unwrap();

        list.push(Mapping {
            start: source_start,
            dest_start: destination_start,
            range,
        });

        row_index += 1;
    }

    (list, row_index)
}

// generic function that takes input ranges and maps them into output ranges
fn map_ranges(input_ranges: &Vec<Range>, mappings: &Vec<Mapping>) -> Vec<Range> {
    input_ranges.iter()
        .map(|input_range| map_range(input_range, &mappings))
        .flatten()
        .collect::<Vec<Range>>()
}

fn map_range(input_range: &Range, sorted_mappings: &Vec<Mapping>) -> Vec<Range> {
    let mut new_ranges = Vec::new();

    let mut start = input_range.start;
    let mappings_in_range = sorted_mappings.into_iter()
        .filter(|mapping| mapping_contains_range(input_range, mapping))
        .collect::<Vec<&Mapping>>();

    for mapping in &mappings_in_range {
       println!("filtered mappings = {} - {}", mapping.start, mapping.range);
       println!("input range = {} - {}", start, input_range.end);

        if start < mapping.start {
            new_ranges.push(Range { start, end: mapping.start - 1 });
            start = mapping.start;
        }

        if start <= input_range.end {
            let range_start = start - mapping.start;
            let range_end = min(input_range.end, mapping.start + mapping.range - 1) - mapping.start;

            new_ranges.push(Range {
                start: mapping.dest_start + range_start,
                end: mapping.dest_start + range_end
            });

            start = mapping.start + mapping.range
        }
    }

    if start <= input_range.end {
        new_ranges.push(Range { start, end: input_range.end })
    }

    new_ranges
}

fn mapping_contains_range(range: &Range, mapping: &Mapping) -> bool {
    in_range(range, mapping.start)
        || in_range(range, mapping.start + mapping.range - 1)
        || in_range(&Range { start: mapping.start, end: mapping.start + mapping.range - 1 }, range.start)
        || in_range(&Range { start: mapping.start, end: mapping.start + mapping.range - 1 }, range.end)
}

fn in_range(range: &Range, value: usize) -> bool {
    value >= range.start && value <= range.end
}