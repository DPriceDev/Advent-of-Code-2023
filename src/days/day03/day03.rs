use std::collections::HashMap;
use std::time::{Instant};
use crate::utils::file as file;

pub fn run() {
    println!("day 3");
    let now = Instant::now();
    println!("  part 1: {} in {}ms", part_one(), now.elapsed().as_millis());
    let now = Instant::now();
    println!("  part 2: {} in {}ms", part_two(), now.elapsed().as_millis());
}

fn part_one() -> u32 {
    let input_grid = file::get_lines("input/day03/input.txt");
    return get_symbols(input_grid).into_iter()
        .map(|symbol| symbol.numbers.into_iter().sum::<u32>())
        .sum::<u32>();
}

fn part_two() -> u32 {
    let input_grid = file::get_lines("input/day03/input.txt");
    get_symbols(input_grid).into_iter()
        .filter(|symbol| symbol.symbol == '*' && symbol.numbers.len() > 1)
        .map(|symbol| symbol.numbers.into_iter().reduce(|a, b| a * b).unwrap())
        .sum::<u32>()
}

struct Number {
    value: u32,
    id: usize,
}

struct Symbol {
    symbol: char,
    numbers: Vec<u32>,
}

fn get_symbols(input_grid: Vec<String>) -> Vec<Symbol> {
    let mut id = 0;
    let mut numbers: HashMap<(usize, usize), Number> = HashMap::new();
    let mut symbols: HashMap<(usize, usize), Symbol> = HashMap::new();

    for (y, row) in input_grid.iter().enumerate() {
        let mut x = 0;
        while x != row.len() {
            let entry = row.chars().nth(x).unwrap();

            if entry.is_digit(10) {
                let (number, length) = parse_number(&row[x..row.len()]);

                for x_offset in 0..length {
                    let key = (x + x_offset, y);
                    numbers.insert(key, Number { value: number, id });
                }
                id += 1;

                let to_check = get_surrounding_indices(x, y, length);

                for key in to_check {
                    if symbols.contains_key(&key) {
                        let symbol = symbols.get(&key).unwrap();
                        let mut new_numbers = symbol.numbers.clone();
                        new_numbers.push(number);

                        let new_symbol = Symbol { symbol: symbol.symbol, numbers: new_numbers };
                        symbols.insert(key, new_symbol);
                    }
                }

                x += length - 1;
            } else if entry != '.' {
                let key = (x, y);

                let to_check = get_surrounding_indices(x, y, 0);
                let mut ids = Vec::new();

                let mut adjacent_numbers = Vec::new();

                for key in to_check {
                    if numbers.contains_key(&key) {
                        let number = numbers.get(&key).unwrap();
                        if !ids.contains(&number.id) {
                            ids.push(number.id);
                            adjacent_numbers.push(number.value)
                        }
                    }
                }

                let symbol = Symbol { symbol: entry, numbers: Vec::from(adjacent_numbers) };

                symbols.insert(key, symbol);
            }

            x += 1;
        }
    }

    symbols.into_iter().map(|(_, symbol)| symbol).collect()
}

fn get_surrounding_indices(x: usize, y: usize, offset: usize) -> Vec<(usize, usize)> {
    let mut indices = Vec::new();
    if x > 0 && y > 0 { indices.push((x - 1, y - 1)); }
    if x > 0 { indices.push((x - 1, y)); }

    if y > 0 {
        indices.push((x, y - 1));

        if offset == 0 {
            indices.push((x + 1, y - 1));
        } else {
            for value in 0..offset {
                indices.push((x + 1 + value, y - 1));
            }
        }
    }
    indices
}

fn parse_number(row: &str) -> (u32, usize) {
    let str = row.chars()
        .take_while(|entry| entry.is_digit(10))
        .collect::<String>();

    (str.parse::<u32>().unwrap(), str.len())
}