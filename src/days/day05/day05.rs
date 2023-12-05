use std::cmp::{min};
use crate::utils::file as file;

use regex::Regex;
use std::time::{Instant};
use once_cell::sync::{Lazy};

pub fn run() {
    println!("day 5");
    let now = Instant::now();
    println!("  part 1: {} in {}ms", part_one(), now.elapsed().as_millis());
    let now = Instant::now();
    println!("  part 2: {} in {}ms", part_two(), now.elapsed().as_millis());
}

struct Mapping {
    source_start: usize,
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
    static MAPPING_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+) (\d+) (\d+)").unwrap());
    almanac.split(|row| row.is_empty() || row.chars().any(|c| c.is_alphabetic()))
        .filter(|group| !group.is_empty())
        .map(Vec::from)
        .map(|mapping_group| {
            mapping_group.iter().map(|entry| {
               let (_, [destination, source, range]) = MAPPING_REGEX.captures_iter(&*entry)
                   .nth(0)
                   .unwrap()
                   .extract();

                Mapping {
                    source_start: source.parse::<usize>().unwrap(),
                    dest_start: destination.parse::<usize>().unwrap(),
                    range: range.parse::<usize>().unwrap(),
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
    let (start, mut ranges) = sorted_mappings.iter()
        .filter(|mapping| mapping_contains_range(input_range, mapping))
        .fold((input_range.start, Vec::new()), |(mut start, mut ranges), mapping| {
            if start < mapping.source_start {
                ranges.push(Range { start, end: mapping.source_start - 1 });
                start = mapping.source_start;
            }

            if start <= input_range.end {
                let range_end = min(input_range.end, mapping.source_start + mapping.range - 1) - mapping.source_start;
                let range = Range {
                    start: mapping.dest_start +  start - mapping.source_start,
                    end: mapping.dest_start + range_end,
                };

                ranges.push(range);
                start = mapping.source_start + mapping.range
            }

            (start, ranges)
        });

    if start <= input_range.end {
        ranges.push(Range { start, end: input_range.end })
    }

    ranges
}

fn mapping_contains_range(range: &Range, mapping: &Mapping) -> bool {
    in_range(range, mapping.source_start)
        || in_range(range, mapping.source_start + mapping.range - 1)
        || in_range(&Range { start: mapping.source_start, end: mapping.source_start + mapping.range - 1 }, range.start)
        || in_range(&Range { start: mapping.source_start, end: mapping.source_start + mapping.range - 1 }, range.end)
}

fn in_range(range: &Range, value: usize) -> bool {
    value >= range.start && value <= range.end
}