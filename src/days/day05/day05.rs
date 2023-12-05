use std::cmp::Ordering;
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

struct Range {
    start: usize,
    dest_start: usize,
    range: usize,
}

fn part_one() -> u32 {
    let almanac = file::get_lines("input/day05/input.txt");

    let seeds = Regex::new(r"(\d+)").unwrap()
        .find_iter(almanac.clone().into_iter().nth(0).unwrap().as_str())
        .map(|seed| seed.as_str().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut products = HashMap::new();

    let mut row_index = 2;

    while row_index < almanac.clone().len() {
        row_index = parse_product(row_index, &almanac, &mut products);
    }

    let test = find_locations(&seeds, &products);
    let (_, min) = test.iter().min_by(|(_, a), (_, b)| a.cmp(b) ).unwrap();

    min.clone() as u32
}

fn parse_product(index: usize, almanac: &Vec<String>, products: &mut HashMap<String, Product>) -> usize {
    static NAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w+)-to-(\w+)").unwrap());

    let mut row_index = index;

    let name_row =  almanac.into_iter().nth(row_index).unwrap();
    let (_, [source, destination]) = NAME_REGEX.captures_iter(name_row).nth(0).unwrap().extract();

    let mut list = Vec::new();
    row_index += 1;
    static MAPPING_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
    while row_index < almanac.len() {
        let row =  almanac.into_iter().nth(row_index).unwrap();

        if row.is_empty() {
            row_index += 1;
            products.insert(source.to_string(), Product { next: destination.to_string(), ranges: list });
            return row_index
        }

        let destination_start = MAPPING_REGEX.find_iter(row).nth(0).unwrap().as_str().parse::<usize>().unwrap();
        let source_start = MAPPING_REGEX.find_iter(row).nth(1).unwrap().as_str().parse::<usize>().unwrap();
        let range = MAPPING_REGEX.find_iter(row).nth(2).unwrap().as_str().parse::<usize>().unwrap();

        list.push(Range {
            start: source_start,
            dest_start: destination_start,
            range,
        });

        row_index += 1;
    }

    products.insert(source.to_string(), Product { next: destination.to_string(), ranges: list });
    row_index
}

fn find_locations(seeds: &Vec<u32>, products: &HashMap<String, Product>) -> Vec<(u32, usize)> {

    let mut seed_locations = Vec::new();
    for seed in seeds {
        let mut current_product = "seed";
        let mut current_index = seed.clone() as usize;

        while current_product != "location" {
            let product = products.get(current_product).unwrap();

            current_index = test(&mut current_index, &product);

            current_product = &*product.next;
        }

        // save location
        seed_locations.push((*seed, current_index));
    }

    seed_locations
}

fn test(current_index: & usize, product: &Product) -> usize {
    for range in &product.ranges {
        let source_max = range.start + range.range;
        if current_index >= &range.start && current_index < &source_max {
            return range.dest_start + (*current_index - range.start);
        }
    }
    *current_index
}

fn part_two() -> u32 {
    0
}