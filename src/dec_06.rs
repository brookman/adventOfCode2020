use std::collections::HashMap;

use itertools::Itertools;

use crate::common;

pub fn part_one() {
    println!("--- Part One ---");

    let sum: usize = common::read_chunks("./data/dec_06.txt").iter()
        .map(|c| c.iter().flat_map(|l| l.chars()).unique().count())
        .sum();
    println!("Result: {}", sum);
}

pub fn part_two() {
    println!("--- Part Two ---");

    let sum: usize = common::read_chunks("./data/dec_06.txt").iter()
        .map(|chunk| (chunk_to_count_map(chunk), chunk.len() as i32))
        .map(|(char_counts, len)| {
            char_counts.iter()
                .filter(|(_, &value)| value == len)
                .count()
        }).sum();
    println!("Result: {}", sum);
}

fn chunk_to_count_map(chunk: &Vec<String>) -> HashMap<char, i32> {
    return to_count_map(&chars_from_chunk(chunk));
}

fn to_count_map(chars: &Vec<char>) -> HashMap<char, i32> {
    let mut map: HashMap<char, i32> = HashMap::new();
    for c in chars {
        map.insert(*c, map.get(&c).unwrap_or(&0) + 1);
    }
    return map;
}

fn chars_from_chunk(chunk: &Vec<String>) -> Vec<char> {
    return chunk.iter().flat_map(|l| l.chars()).collect::<Vec<char>>();
}
