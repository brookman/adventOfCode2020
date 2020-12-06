use std::collections::{HashMap, HashSet};

use crate::common;

pub fn part_one() {
    println!("--- Part One ---");

    let sum: i32 = common::read_chunks("./data/dec_06.txt").iter()
        .map(|chunk| {
            let mut set: HashSet<char> = HashSet::new();
            for line in chunk {
                for c in line.chars() {
                    set.insert(c);
                }
            }
            return set.len() as i32;
        }).sum();

    println!("Result: {}", sum);
}

pub fn part_two() {
    println!("--- Part Two ---");

    let chunks = common::read_chunks("./data/dec_06.txt");
    let mut sum = 0;
    for chunk in chunks {
        let mut map: HashMap<char, i32> = HashMap::new();
        let chunk_len = chunk.len() as i32;
        for line in chunk {
            for c in line.chars() {
                map.insert(c, map.get(&c).unwrap_or(&0) + 1);
            }
        }
        sum += map.iter().filter(|(&_, &value)| value == chunk_len).count();
    }

    println!("Result: {}", sum);
}
