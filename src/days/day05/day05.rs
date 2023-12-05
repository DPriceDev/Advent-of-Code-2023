use std::cmp::{min};
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

    let seeds = get_seeds(almanac.iter().nth(0).unwrap().as_str())
        .iter()
        .map(|seeds| Range { start: *seeds, end: *seeds })
        .collect::<Vec<Range>>();

    get_lowest_seed(&almanac, seeds)
}

fn part_two() -> usize {
    let almanac = file::get_lines("input/day05/input.txt");

    let seeds = get_seeds(almanac.iter().nth(0).unwrap().as_str())
        .chunks(2)
        .map(|seeds| {
            let start = *seeds.iter().nth(0).unwrap();
            Range { start, end: start + *seeds.iter().nth(1).unwrap() }
        })
        .collect::<Vec<Range>>();

    get_lowest_seed(&almanac, seeds)
}

fn get_seeds(input: &str) -> Vec<usize> {
    Regex::new(r"(\d+)").unwrap()
        .find_iter(input)
        .map(|seed| seed.as_str().parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn get_lowest_seed(almanac: &Vec<String>, seeds: Vec<Range>) -> usize {
    get_mappings(&almanac).iter()
        .fold(seeds, |running, mapping| map_ranges(&running, &mapping))
        .iter()
        .min_by(|a, b| a.start.cmp(&b.start))
        .unwrap()
        .start
}

fn get_mappings(almanac: &Vec<String>) -> Vec<Vec<Mapping>> {
    static MAPPING_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
    almanac.split(|row| row.is_empty() || row.chars().any(|c| c.is_alphabetic()))
        .filter(|group| !group.is_empty())
        .map(Vec::from)
        .map(|mapping_group| {
            mapping_group.iter().map(|entry| {
                Mapping {
                    start: MAPPING_REGEX.find_iter(&*entry).nth(1).unwrap().as_str().parse::<usize>().unwrap(),
                    dest_start: MAPPING_REGEX.find_iter(&*entry).nth(0).unwrap().as_str().parse::<usize>().unwrap(),
                    range: MAPPING_REGEX.find_iter(&*entry).nth(2).unwrap().as_str().parse::<usize>().unwrap(),
                }
            }).collect::<Vec<Mapping>>()
        })
        .collect::<Vec<Vec<Mapping>>>()
}

fn map_ranges(input_ranges: &Vec<Range>, mappings: &Vec<Mapping>) -> Vec<Range> {
    input_ranges.iter()
        .map(|input_range| map_range(input_range, &mappings))
        .flatten()
        .collect::<Vec<Range>>()
}

fn map_range(input_range: &Range, sorted_mappings: &Vec<Mapping>) -> Vec<Range> {
    let mut new_ranges = Vec::new();

    let mut start = input_range.start;
    let mappings_in_range = sorted_mappings.iter()
        .filter(|mapping| mapping_contains_range(input_range, mapping))
        .collect::<Vec<&Mapping>>();

    for mapping in &mappings_in_range {
        if start < mapping.start {
            new_ranges.push(Range { start, end: mapping.start - 1 });
            start = mapping.start;
        }

        if start <= input_range.end {
            let range_start = start - mapping.start;
            let range_end = min(input_range.end, mapping.start + mapping.range - 1) - mapping.start;

            new_ranges.push(Range {
                start: mapping.dest_start + range_start,
                end: mapping.dest_start + range_end,
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