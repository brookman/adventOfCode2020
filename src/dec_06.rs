use std::collections::{HashMap, HashSet};

use crate::common;

pub fn part_one() {
    println!("--- Part One ---");

    let lines = common::read_strings("./data/dec_06.txt");
    let mut sum = 0;
    let mut set: HashSet<char> = HashSet::new();
    for line in lines {
        if line.trim().is_empty() {
            sum += set.len();
            set.clear();
        } else {
            for c in line.chars() {
                set.insert(c);
            }
        }
    }
    sum += set.len();
    set.clear();

    println!("Result: {}", sum);
}

pub fn part_two() {
    println!("--- Part Two ---");

    let lines = common::read_strings("./data/dec_06.txt");
    let mut sum = 0;
    let mut count = 0;
    let mut map: HashMap<char, i32> = HashMap::new();
    for line in lines {
        if line.trim().is_empty() {
            sum += map.iter().filter(|(&key,&value)|value == count).count();
            map.clear();
            count = 0;
        } else {
            for c in line.chars() {
                map.insert(c, map.get(&c).unwrap_or(&0) + 1);
            }
            count += 1;
        }

    }
    sum += map.iter().filter(|(&key,&value)|value == count).count();
    map.clear();
    count = 0;

    println!("Result: {}", sum);
}
